// Main game logic and UI controller

class BattleForMoscowGame {
    constructor() {
        this.gameState = null;
        this.mapData = null;
        this.unitsData = null;
        this.selectedUnit = null;
        this.validMoves = [];
        this.tooltip = null;
        this.currentMode = 'move'; // move, battle, retreat, replacement

        this.init();
    }

    async init() {
        // Load initial data
        await this.loadMapData();
        await this.loadUnitsData();
        await this.loadGameState();

        // Create tooltip element
        this.createTooltip();

        // Setup event listeners
        this.setupEventListeners();

        // Render initial state
        this.render();

        this.log('Game initialized. Click "New Game" to start.');
    }

    createTooltip() {
        this.tooltip = document.createElement('div');
        this.tooltip.className = 'hex-tooltip';
        this.tooltip.style.display = 'none';
        document.body.appendChild(this.tooltip);

        this.combatPreview = document.createElement('div');
        this.combatPreview.className = 'combat-preview';
        this.combatPreview.style.display = 'none';
        document.body.appendChild(this.combatPreview);
    }

    showTooltip(hex, event) {
        if (!hex) return;

        const terrain = hex.terrain || 'clear';
        const modifiers = this.getHexModifiers(hex);

        let content = `<h4>Hex (${hex.q}, ${hex.r})</h4>`;
        content += `<p><strong>Terrain:</strong> ${terrain.charAt(0).toUpperCase() + terrain.slice(1)}</p>`;

        if (hex.city) {
            content += `<p><strong>City:</strong> ${hex.city.name}</p>`;
        }

        if (hex.fortification) {
            content += `<p class="modifier">Fortified</p>`;
        }

        if (modifiers.length > 0) {
            content += `<p><strong>Combat Modifiers:</strong></p>`;
            modifiers.forEach(mod => {
                content += `<p class="modifier">• ${mod}</p>`;
            });
        }

        this.tooltip.innerHTML = content;
        this.tooltip.style.display = 'block';
        this.tooltip.style.left = (event.pageX + 15) + 'px';
        this.tooltip.style.top = (event.pageY + 15) + 'px';
    }

    hideTooltip() {
        if (this.tooltip) {
            this.tooltip.style.display = 'none';
        }
    }

    showCombatPreview(attackerUnit, defenderUnit, event) {
        if (!attackerUnit || !defenderUnit) return;

        const attackerDef = this.unitsData.units.find(u => u.id === attackerUnit.id);
        const defenderDef = this.unitsData.units.find(u => u.id === defenderUnit.id);

        // Check if they're enemies
        if (attackerDef.side === defenderDef.side) return;

        const attackerStrength = attackerDef[attackerUnit.strength === 'full' ? 'full_strength' : 'half_strength'];
        const defenderStrength = defenderDef[defenderUnit.strength === 'full' ? 'full_strength' : 'half_strength'];

        const odds = this.calculateOdds(attackerStrength, defenderStrength);

        this.combatPreview.innerHTML = `
            <strong>Attack:</strong> ${attackerUnit.id} (${attackerStrength})<br>
            vs ${defenderUnit.id} (${defenderStrength})<br>
            <strong>Odds: ${odds}</strong>
        `;
        this.combatPreview.style.display = 'block';
        this.combatPreview.style.left = (event.pageX + 15) + 'px';
        this.combatPreview.style.top = (event.pageY + 15) + 'px';
    }

    hideCombatPreview() {
        if (this.combatPreview) {
            this.combatPreview.style.display = 'none';
        }
    }

    getHexModifiers(hex) {
        const modifiers = [];

        if (hex.terrain === 'forest') {
            modifiers.push('Attacker -1 column');
        }

        if (hex.fortification) {
            modifiers.push('Defender +1 column');
        }

        if (hex.city) {
            if (hex.city.name === 'Moscow') {
                modifiers.push('Moscow: Attacker -1 column');
            }
        }

        return modifiers;
    }

    async loadMapData() {
        const response = await fetch('/api/map');
        const result = await response.json();
        if (result.success) {
            this.mapData = result.data;
        }
    }

    async loadUnitsData() {
        const response = await fetch('/api/units');
        const result = await response.json();
        if (result.success) {
            this.unitsData = result.data;
        }
    }

    async loadGameState() {
        const response = await fetch('/api/game');
        const result = await response.json();
        if (result.success) {
            this.gameState = result.data;
            this.updateStatusDisplay();
        }
    }

    setupEventListeners() {
        document.getElementById('new-game-btn').addEventListener('click', () => this.newGame());
        document.getElementById('advance-phase-btn').addEventListener('click', () => this.advancePhase());
        document.getElementById('declare-battle-btn').addEventListener('click', () => this.openBattleDialog());
        document.getElementById('apply-replacement-btn').addEventListener('click', () => this.showReplacementPanel());
    }

    async newGame() {
        const response = await fetch('/api/game/new', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({})
        });

        const result = await response.json();
        if (result.success) {
            this.gameState = result.data;
            this.selectedUnit = null;
            this.validMoves = [];
            this.render();
            this.log('New game started!');
        } else {
            this.log('Error: ' + result.error, 'error');
        }
    }

    async advancePhase() {
        const response = await fetch('/api/game/advance-phase', {
            method: 'POST'
        });

        const result = await response.json();
        if (result.success) {
            this.gameState = result.data;
            this.selectedUnit = null;
            this.validMoves = [];
            this.render();
            this.log(`Advanced to ${this.gameState.phase.replace(/_/g, ' ')}`);
        } else {
            this.log('Error: ' + result.error, 'error');
        }
    }

    async selectUnit(unitId) {
        this.selectedUnit = unitId;

        // Load valid moves for this unit
        const response = await fetch(`/api/units/${unitId}/valid-moves`);
        const result = await response.json();

        if (result.success) {
            this.validMoves = result.data || [];
        } else {
            this.validMoves = [];
        }

        this.render();
    }

    async moveUnit(q, r) {
        if (!this.selectedUnit) return;

        const response = await fetch('/api/units/move', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                unit_id: this.selectedUnit,
                to_q: q,
                to_r: r
            })
        });

        const result = await response.json();
        if (result.success) {
            this.gameState = result.data;
            this.selectedUnit = null;
            this.validMoves = [];
            this.render();
            this.log(`Unit ${this.selectedUnit} moved to (${q}, ${r})`);
        } else {
            this.log('Move error: ' + result.error, 'error');
        }
    }

    updateStatusDisplay() {
        if (!this.gameState) return;

        document.getElementById('turn-display').textContent = this.gameState.turn;
        document.getElementById('phase-display').textContent =
            this.gameState.phase.replace(/_/g, ' ').replace(/\b\w/g, l => l.toUpperCase());

        const activePlayer = this.gameState.phase.includes('german') ? 'German' : 'Soviet';
        document.getElementById('player-display').textContent = activePlayer;
        document.getElementById('player-display').style.color =
            activePlayer === 'German' ? '#888' : '#c41e3a';

        const isMud = this.gameState.turn === 3 || this.gameState.turn === 4;
        document.getElementById('mud-display').textContent = isMud ? 'Yes' : 'No';
        document.getElementById('mud-display').style.color = isMud ? '#ff9800' : '#4caf50';

        // Update victory conditions
        this.updateVictoryConditions();

        // Show/hide replacement button based on phase
        const isReplacementPhase = this.gameState.phase.includes('replacement');
        document.getElementById('apply-replacement-btn').style.display =
            isReplacementPhase ? 'block' : 'none';
    }

    updateVictoryConditions() {
        if (!this.gameState) return;

        const panel = document.getElementById('victory-conditions-panel');
        const container = document.getElementById('victory-conditions');

        // Show panel after turn 1
        if (this.gameState.turn > 1) {
            panel.style.display = 'block';
        } else {
            panel.style.display = 'none';
            return;
        }

        const conditions = this.calculateVictoryConditions();

        let html = '';
        conditions.forEach(condition => {
            const statusClass = condition.achieved ? 'achieved' : (condition.failed ? 'failed' : 'pending');
            html += `
                <div class="victory-condition ${statusClass}">
                    <strong>${condition.name}:</strong> ${condition.description}
                </div>
            `;
        });

        container.innerHTML = html;
    }

    calculateVictoryConditions() {
        const conditions = [];

        // Check if Moscow is controlled
        const moscowHex = this.mapData.hexes.find(h => h.city && h.city.name === 'Moscow');
        let moscowController = 'Soviet'; // Default

        if (moscowHex && this.gameState.units) {
            const moscowUnit = this.gameState.units.find(u =>
                u.position && u.position[0] === moscowHex.q && u.position[1] === moscowHex.r
            );

            if (moscowUnit) {
                const unitDef = this.unitsData.units.find(u => u.id === moscowUnit.id);
                moscowController = unitDef.side === 'german' ? 'German' : 'Soviet';
            }
        }

        conditions.push({
            name: 'Moscow Control',
            description: `Currently controlled by ${moscowController}`,
            achieved: moscowController === 'Soviet',
            failed: moscowController === 'German'
        });

        // Game end condition
        const isGameOver = this.gameState.turn > 7;
        conditions.push({
            name: 'Game Duration',
            description: `Turn ${this.gameState.turn} of 7`,
            achieved: !isGameOver,
            failed: false
        });

        // Victory determination
        if (isGameOver) {
            conditions.push({
                name: 'Final Result',
                description: moscowController === 'Soviet' ? 'Soviet Victory!' : 'German Victory!',
                achieved: moscowController === 'Soviet',
                failed: moscowController === 'German'
            });
        }

        return conditions;
    }

    render() {
        this.updateStatusDisplay();
        this.renderMap();
        this.renderUnitDetails();
    }

    renderMap() {
        const svg = document.getElementById('game-map');
        svg.innerHTML = '';

        if (!this.mapData) return;

        // Calculate viewBox
        const bounds = calculateMapBounds(this.mapData.hexes);
        svg.setAttribute('viewBox',
            `${bounds.minX} ${bounds.minY} ${bounds.maxX - bounds.minX} ${bounds.maxY - bounds.minY}`);

        // Render hexes
        for (const hex of this.mapData.hexes) {
            const hexEl = createHexElement(hex.q, hex.r, hex.terrain, {
                city: hex.city,
                fortification: hex.fortification
            });

            // Add tooltip event listeners
            hexEl.addEventListener('mouseenter', (e) => this.showTooltip(hex, e));
            hexEl.addEventListener('mousemove', (e) => {
                if (this.tooltip.style.display === 'block') {
                    this.tooltip.style.left = (e.pageX + 15) + 'px';
                    this.tooltip.style.top = (e.pageY + 15) + 'px';
                }
            });
            hexEl.addEventListener('mouseleave', () => this.hideTooltip());

            // Check if this is a valid move destination
            const isValidMove = this.validMoves.some(m => m.q === hex.q && m.r === hex.r);
            if (isValidMove) {
                hexEl.querySelector('.hex').classList.add('hex-valid-move');
                hexEl.style.cursor = 'pointer';
                hexEl.addEventListener('click', () => this.moveUnit(hex.q, hex.r));
            }

            svg.appendChild(hexEl);
        }

        // Render units
        if (this.gameState && this.gameState.units) {
            for (const unit of this.gameState.units) {
                if (!unit.position) continue;

                const [q, r] = unit.position;
                const unitDef = this.unitsData.units.find(u => u.id === unit.id);
                if (!unitDef) continue;

                const unitEl = createUnitElement(q, r, unit, unitDef);

                // Highlight selected unit
                if (this.selectedUnit === unit.id) {
                    unitEl.querySelector('.unit').classList.add('hex-selected');
                }

                // Add click handler
                unitEl.style.cursor = 'pointer';
                unitEl.addEventListener('click', (e) => {
                    e.stopPropagation();
                    this.selectUnit(unit.id);
                });

                // Add combat preview on hover
                unitEl.addEventListener('mouseenter', (e) => {
                    if (this.selectedUnit && this.selectedUnit !== unit.id) {
                        const selectedUnit = this.gameState.units.find(u => u.id === this.selectedUnit);
                        if (selectedUnit) {
                            this.showCombatPreview(selectedUnit, unit, e);
                        }
                    }
                });

                unitEl.addEventListener('mousemove', (e) => {
                    if (this.combatPreview.style.display === 'block') {
                        this.combatPreview.style.left = (e.pageX + 15) + 'px';
                        this.combatPreview.style.top = (e.pageY + 15) + 'px';
                    }
                });

                unitEl.addEventListener('mouseleave', () => {
                    this.hideCombatPreview();
                });

                svg.appendChild(unitEl);
            }
        }
    }

    renderUnitDetails() {
        const container = document.getElementById('unit-details');
        const battleBtn = document.getElementById('declare-battle-btn');

        if (!this.selectedUnit) {
            container.innerHTML = '<p class="placeholder">Select a unit on the map</p>';
            battleBtn.style.display = 'none';
            return;
        }

        const unit = this.gameState.units.find(u => u.id === this.selectedUnit);
        const unitDef = this.unitsData.units.find(u => u.id === this.selectedUnit);

        if (!unit || !unitDef) {
            container.innerHTML = '<p class="placeholder">Unit not found</p>';
            battleBtn.style.display = 'none';
            return;
        }

        const strength = unitDef[unit.strength === 'full' ? 'full_strength' : 'half_strength'];

        // Check if unit can attack
        const adjacentEnemies = unit.position ? this.findAdjacentEnemies(unit) : [];
        const isCombatPhase = this.gameState && this.gameState.phase && this.gameState.phase.includes('combat');

        if (adjacentEnemies.length > 0 && isCombatPhase) {
            battleBtn.style.display = 'block';
        } else {
            battleBtn.style.display = 'none';
        }

        container.innerHTML = `
            <div style="margin-bottom: 10px;">
                <strong style="font-size: 1.2em; color: ${unitDef.side === 'german' ? '#888' : '#c41e3a'}">
                    ${unit.id}
                </strong>
            </div>
            <p><strong>Side:</strong> ${unitDef.side.charAt(0).toUpperCase() + unitDef.side.slice(1)}</p>
            <p><strong>Type:</strong> ${unitDef.type.charAt(0).toUpperCase() + unitDef.type.slice(1)}</p>
            <p><strong>Strength:</strong> ${strength} (${unit.strength})</p>
            <p><strong>Movement:</strong> ${unitDef.movement}</p>
            ${unit.position ? `<p><strong>Position:</strong> (${unit.position[0]}, ${unit.position[1]})</p>` : ''}
            <p style="margin-top: 10px; color: #4caf50;">
                <strong>Valid moves:</strong> ${this.validMoves.length}
            </p>
            ${adjacentEnemies.length > 0 ? `
                <p style="margin-top: 8px; color: #e74c3c;">
                    <strong>Adjacent enemies:</strong> ${adjacentEnemies.length}
                </p>
            ` : ''}
        `;
    }

    log(message, type = 'info') {
        const logContainer = document.getElementById('game-log');
        const entry = document.createElement('div');
        entry.className = 'log-entry';
        entry.textContent = `[${new Date().toLocaleTimeString()}] ${message}`;

        if (type === 'error') {
            entry.style.borderLeftColor = '#e74c3c';
        }

        logContainer.insertBefore(entry, logContainer.firstChild);

        // Keep only last 20 entries
        while (logContainer.children.length > 20) {
            logContainer.removeChild(logContainer.lastChild);
        }
    }

    // Battle Declaration Interface
    openBattleDialog() {
        if (!this.selectedUnit) {
            this.log('Please select a unit first', 'error');
            return;
        }

        const unit = this.gameState.units.find(u => u.id === this.selectedUnit);
        if (!unit || !unit.position) return;

        // Find adjacent enemy units
        const adjacentEnemies = this.findAdjacentEnemies(unit);

        if (adjacentEnemies.length === 0) {
            this.log('No adjacent enemy units to attack', 'error');
            return;
        }

        this.showBattlePanel(unit, adjacentEnemies);
    }

    findAdjacentEnemies(unit) {
        const unitDef = this.unitsData.units.find(u => u.id === unit.id);
        const [q, r] = unit.position;

        const neighbors = [
            [q + 1, r], [q - 1, r],
            [q, r + 1], [q, r - 1],
            [q + 1, r - 1], [q - 1, r + 1]
        ];

        const enemies = [];
        for (const unit of this.gameState.units) {
            if (!unit.position) continue;
            const [uq, ur] = unit.position;
            const def = this.unitsData.units.find(u => u.id === unit.id);

            if (def.side !== unitDef.side && neighbors.some(([nq, nr]) => nq === uq && nr === ur)) {
                enemies.push({ unit, def });
            }
        }

        return enemies;
    }

    showBattlePanel(attacker, defenders) {
        const overlay = document.createElement('div');
        overlay.className = 'modal-overlay';
        overlay.onclick = () => this.closeBattlePanel();

        const panel = document.createElement('div');
        panel.className = 'battle-panel';
        panel.onclick = (e) => e.stopPropagation();

        const attackerDef = this.unitsData.units.find(u => u.id === attacker.id);
        const attackerStrength = attackerDef[attacker.strength === 'full' ? 'full_strength' : 'half_strength'];

        let defenderStrength = 0;
        let defenderHTML = '';

        defenders.forEach(({ unit, def }) => {
            const strength = def[unit.strength === 'full' ? 'full_strength' : 'half_strength'];
            defenderStrength += strength;
            defenderHTML += `
                <div class="unit-list-item">
                    <strong>${unit.id}</strong>: ${strength}
                </div>
            `;
        });

        const odds = this.calculateOdds(attackerStrength, defenderStrength);

        panel.innerHTML = `
            <h2>Declare Battle</h2>
            <div class="battle-forces">
                <div class="force-column">
                    <h3 style="color: #888">Attacker</h3>
                    <div class="unit-list-item">
                        <strong>${attacker.id}</strong>: ${attackerStrength}
                    </div>
                    <p style="margin-top: 10px; text-align: center; font-size: 1.2em">
                        <strong>Total: ${attackerStrength}</strong>
                    </p>
                </div>
                <div class="odds-display">
                    ${odds}
                </div>
                <div class="force-column">
                    <h3 style="color: #c41e3a">Defender</h3>
                    ${defenderHTML}
                    <p style="margin-top: 10px; text-align: center; font-size: 1.2em">
                        <strong>Total: ${defenderStrength}</strong>
                    </p>
                </div>
            </div>
            <div class="battle-buttons">
                <button class="btn btn-secondary" onclick="window.game.closeBattlePanel()">Cancel</button>
                <button class="btn btn-danger" onclick="window.game.declareBattle()">Declare Battle</button>
            </div>
        `;

        document.body.appendChild(overlay);
        document.body.appendChild(panel);

        this.currentBattlePanel = { overlay, panel, attacker, defenders };
    }

    closeBattlePanel() {
        if (this.currentBattlePanel) {
            document.body.removeChild(this.currentBattlePanel.overlay);
            document.body.removeChild(this.currentBattlePanel.panel);
            this.currentBattlePanel = null;
        }
    }

    calculateOdds(attackerStrength, defenderStrength) {
        if (defenderStrength === 0) return '6:1+';

        const ratio = attackerStrength / defenderStrength;

        if (ratio >= 6) return '6:1';
        if (ratio >= 5) return '5:1';
        if (ratio >= 4) return '4:1';
        if (ratio >= 3) return '3:1';
        if (ratio >= 2) return '2:1';
        if (ratio >= 1) return '1:1';
        if (ratio >= 0.5) return '1:2';
        if (ratio >= 0.33) return '1:3';
        return '1:4';
    }

    async declareBattle() {
        if (!this.currentBattlePanel) return;

        const { attacker, defenders } = this.currentBattlePanel;
        const defenderIds = defenders.map(d => d.unit.id);

        const response = await fetch('/api/battle/declare', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                attacker_ids: [attacker.id],
                defender_id: defenderIds[0]
            })
        });

        const result = await response.json();

        if (result.success) {
            this.gameState = result.data;
            this.closeBattlePanel();
            this.render();
            this.log('Battle declared successfully');
        } else {
            this.log('Battle error: ' + result.error, 'error');
        }
    }

    // Retreat Path Selection Interface
    async showRetreatPanel(unitId) {
        const response = await fetch(`/api/retreat/${unitId}/valid-hexes`);
        const result = await response.json();

        if (!result.success || !result.data || result.data.length === 0) {
            this.log('No valid retreat hexes', 'error');
            return;
        }

        const validHexes = result.data;

        const overlay = document.createElement('div');
        overlay.className = 'modal-overlay';
        overlay.onclick = () => this.closeRetreatPanel();

        const panel = document.createElement('div');
        panel.className = 'retreat-panel';
        panel.onclick = (e) => e.stopPropagation();

        let hexListHTML = '';
        validHexes.forEach(hex => {
            hexListHTML += `
                <div class="replacement-option" onclick="window.game.executeRetreat('${unitId}', ${hex.q}, ${hex.r})">
                    Hex (${hex.q}, ${hex.r})
                </div>
            `;
        });

        panel.innerHTML = `
            <h2>Select Retreat Path</h2>
            <p>Unit <strong>${unitId}</strong> must retreat.</p>
            <p style="margin: 10px 0;">Select a hex to retreat to:</p>
            <div class="replacement-options">
                ${hexListHTML}
            </div>
            <div class="battle-buttons" style="margin-top: 15px;">
                <button class="btn btn-secondary" onclick="window.game.closeRetreatPanel()">Cancel</button>
            </div>
        `;

        document.body.appendChild(overlay);
        document.body.appendChild(panel);

        this.currentRetreatPanel = { overlay, panel };

        // Highlight valid retreat hexes on map
        this.validRetreatHexes = validHexes;
        this.render();
    }

    closeRetreatPanel() {
        if (this.currentRetreatPanel) {
            document.body.removeChild(this.currentRetreatPanel.overlay);
            document.body.removeChild(this.currentRetreatPanel.panel);
            this.currentRetreatPanel = null;
            this.validRetreatHexes = [];
            this.render();
        }
    }

    async executeRetreat(unitId, q, r) {
        const response = await fetch('/api/retreat/execute', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                unit_id: unitId,
                to_q: q,
                to_r: r
            })
        });

        const result = await response.json();

        if (result.success) {
            this.gameState = result.data;
            this.closeRetreatPanel();
            this.log(`Unit ${unitId} retreated to (${q}, ${r})`);
        } else {
            this.log('Retreat error: ' + result.error, 'error');
        }
    }

    // Replacement Point Application Interface
    async showReplacementPanel() {
        const response = await fetch('/api/replacement/valid-hexes');
        const result = await response.json();

        if (!result.success || !result.data || result.data.length === 0) {
            this.log('No valid replacement hexes available', 'error');
            return;
        }

        const validHexes = result.data;

        // Get eliminated or reduced units that can be replaced
        const replacementUnits = this.gameState.units.filter(u =>
            u.strength === 'half' || u.strength === 'eliminated'
        );

        if (replacementUnits.length === 0) {
            this.log('No units available for replacement', 'error');
            return;
        }

        const overlay = document.createElement('div');
        overlay.className = 'modal-overlay';
        overlay.onclick = () => this.closeReplacementPanel();

        const panel = document.createElement('div');
        panel.className = 'replacement-panel';
        panel.onclick = (e) => e.stopPropagation();

        let unitListHTML = '';
        replacementUnits.forEach(unit => {
            const unitDef = this.unitsData.units.find(u => u.id === unit.id);
            unitListHTML += `
                <div class="replacement-option" id="replacement-unit-${unit.id}" onclick="window.game.selectReplacementUnit('${unit.id}')">
                    <strong>${unit.id}</strong> - ${unitDef.side} ${unitDef.type}<br>
                    <small>Current: ${unit.strength}</small>
                </div>
            `;
        });

        panel.innerHTML = `
            <h2>Apply Replacement</h2>
            <p>Select a unit to restore or bring back:</p>
            <div class="replacement-options">
                ${unitListHTML}
            </div>
            <div class="battle-buttons" style="margin-top: 15px;">
                <button class="btn btn-secondary" onclick="window.game.closeReplacementPanel()">Cancel</button>
            </div>
        `;

        document.body.appendChild(overlay);
        document.body.appendChild(panel);

        this.currentReplacementPanel = { overlay, panel, validHexes };
    }

    selectReplacementUnit(unitId) {
        this.selectedReplacementUnit = unitId;

        // Highlight the selected unit
        document.querySelectorAll('.replacement-option').forEach(el => {
            el.classList.remove('selected');
        });
        document.getElementById(`replacement-unit-${unitId}`).classList.add('selected');

        // Now show hex selection
        this.showReplacementHexSelection(unitId);
    }

    showReplacementHexSelection(unitId) {
        if (!this.currentReplacementPanel) return;

        const { panel, validHexes } = this.currentReplacementPanel;

        let hexListHTML = '';
        validHexes.forEach(hex => {
            hexListHTML += `
                <div class="replacement-option" onclick="window.game.applyReplacement('${unitId}', ${hex.q}, ${hex.r})">
                    Hex (${hex.q}, ${hex.r})
                </div>
            `;
        });

        panel.innerHTML = `
            <h2>Apply Replacement</h2>
            <p>Unit: <strong>${unitId}</strong></p>
            <p style="margin: 10px 0;">Select placement hex:</p>
            <div class="replacement-options">
                ${hexListHTML}
            </div>
            <div class="battle-buttons" style="margin-top: 15px;">
                <button class="btn btn-secondary" onclick="window.game.closeReplacementPanel()">Cancel</button>
            </div>
        `;
    }

    closeReplacementPanel() {
        if (this.currentReplacementPanel) {
            document.body.removeChild(this.currentReplacementPanel.overlay);
            document.body.removeChild(this.currentReplacementPanel.panel);
            this.currentReplacementPanel = null;
            this.selectedReplacementUnit = null;
        }
    }

    async applyReplacement(unitId, q, r) {
        const response = await fetch('/api/replacement/apply', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                unit_id: unitId,
                hex_q: q,
                hex_r: r
            })
        });

        const result = await response.json();

        if (result.success) {
            this.gameState = result.data;
            this.closeReplacementPanel();
            this.log(`Replacement applied to ${unitId} at (${q}, ${r})`);
        } else {
            this.log('Replacement error: ' + result.error, 'error');
        }
    }
}

// Initialize game when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    window.game = new BattleForMoscowGame();
});
