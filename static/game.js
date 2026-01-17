// Main game logic and UI controller

class BattleForMoscowGame {
    constructor() {
        this.gameState = null;
        this.mapData = null;
        this.unitsData = null;
        this.selectedUnit = null;
        this.validMoves = [];

        this.init();
    }

    async init() {
        // Load initial data
        await this.loadMapData();
        await this.loadUnitsData();
        await this.loadGameState();

        // Setup event listeners
        this.setupEventListeners();

        // Render initial state
        this.render();

        this.log('Game initialized. Click "New Game" to start.');
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

                svg.appendChild(unitEl);
            }
        }
    }

    renderUnitDetails() {
        const container = document.getElementById('unit-details');

        if (!this.selectedUnit) {
            container.innerHTML = '<p class="placeholder">Select a unit on the map</p>';
            return;
        }

        const unit = this.gameState.units.find(u => u.id === this.selectedUnit);
        const unitDef = this.unitsData.units.find(u => u.id === this.selectedUnit);

        if (!unit || !unitDef) {
            container.innerHTML = '<p class="placeholder">Unit not found</p>';
            return;
        }

        const strength = unitDef[unit.strength === 'full' ? 'full_strength' : 'half_strength'];

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
}

// Initialize game when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    window.game = new BattleForMoscowGame();
});
