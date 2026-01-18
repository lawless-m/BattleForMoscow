// Hex geometry utilities for SVG rendering

const HEX_SIZE = 30;
const HEX_WIDTH = Math.sqrt(3) * HEX_SIZE;
const HEX_HEIGHT = 2 * HEX_SIZE;

/**
 * Convert axial coordinates (q, r) to pixel coordinates (x, y)
 */
function axialToPixel(q, r) {
    const x = HEX_SIZE * (Math.sqrt(3) * q + Math.sqrt(3) / 2 * r);
    const y = HEX_SIZE * (3 / 2 * r);
    return { x, y };
}

/**
 * Generate SVG path for a hexagon
 */
function hexPath(centerX, centerY, size) {
    const points = [];
    for (let i = 0; i < 6; i++) {
        const angle = (Math.PI / 3) * i;
        const x = centerX + size * Math.cos(angle);
        const y = centerY + size * Math.sin(angle);
        points.push(`${x},${y}`);
    }
    return `M ${points.join(' L ')} Z`;
}

/**
 * Create an SVG hex element
 */
function createHexElement(q, r, terrain, options = {}) {
    const { x, y } = axialToPixel(q, r);

    const g = document.createElementNS('http://www.w3.org/2000/svg', 'g');
    g.setAttribute('class', 'hex-group');
    g.setAttribute('data-q', q);
    g.setAttribute('data-r', r);

    // Create hex path
    const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
    path.setAttribute('d', hexPath(x, y, HEX_SIZE));
    path.setAttribute('class', `hex hex-${terrain}`);
    g.appendChild(path);

    // Add terrain pattern/icon
    addTerrainPattern(g, x, y, terrain);

    // Add coordinate label
    if (options.showCoords !== false) {
        const coordText = document.createElementNS('http://www.w3.org/2000/svg', 'text');
        coordText.setAttribute('x', x);
        coordText.setAttribute('y', y + HEX_SIZE - 5);
        coordText.setAttribute('class', 'hex-coord');
        coordText.textContent = `${q},${r}`;
        g.appendChild(coordText);
    }

    // Add city marker if present
    if (options.city) {
        const cityCircle = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
        cityCircle.setAttribute('cx', x);
        cityCircle.setAttribute('cy', y);
        cityCircle.setAttribute('r', 8);
        cityCircle.setAttribute('class', 'city-marker');
        g.appendChild(cityCircle);

        const cityText = document.createElementNS('http://www.w3.org/2000/svg', 'text');
        cityText.setAttribute('x', x);
        cityText.setAttribute('y', y + 20);
        cityText.setAttribute('class', 'city-name');
        cityText.textContent = options.city.name;
        g.appendChild(cityText);
    }

    // Add fortification if present
    if (options.fortification) {
        const fort = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
        fort.setAttribute('cx', x);
        fort.setAttribute('cy', y);
        fort.setAttribute('r', HEX_SIZE - 5);
        fort.setAttribute('class', 'fortification');
        g.appendChild(fort);
    }

    return g;
}

/**
 * Add terrain-specific visual patterns
 */
function addTerrainPattern(group, x, y, terrain) {
    switch(terrain) {
        case 'forest':
            // Add tree symbols
            for (let i = 0; i < 3; i++) {
                const offsetX = (i - 1) * 8;
                const offsetY = (i % 2) * 6 - 3;
                const tree = document.createElementNS('http://www.w3.org/2000/svg', 'polygon');
                tree.setAttribute('points', `${x + offsetX},${y - 5 + offsetY} ${x + offsetX - 3},${y + 2 + offsetY} ${x + offsetX + 3},${y + 2 + offsetY}`);
                tree.setAttribute('fill', '#1a3010');
                tree.setAttribute('opacity', '0.6');
                group.appendChild(tree);
            }
            break;
        case 'mountain':
            // Add mountain triangles
            const mountain = document.createElementNS('http://www.w3.org/2000/svg', 'polygon');
            mountain.setAttribute('points', `${x},${y - 8} ${x - 8},${y + 4} ${x + 8},${y + 4}`);
            mountain.setAttribute('fill', '#5d4e37');
            mountain.setAttribute('opacity', '0.7');
            group.appendChild(mountain);
            break;
        case 'swamp':
            // Add wavy lines for swamp
            for (let i = -1; i <= 1; i++) {
                const wave = document.createElementNS('http://www.w3.org/2000/svg', 'path');
                wave.setAttribute('d', `M ${x - 10},${y + i * 6} Q ${x - 5},${y + i * 6 - 2} ${x},${y + i * 6} Q ${x + 5},${y + i * 6 + 2} ${x + 10},${y + i * 6}`);
                wave.setAttribute('stroke', '#4a5f42');
                wave.setAttribute('stroke-width', '1');
                wave.setAttribute('fill', 'none');
                wave.setAttribute('opacity', '0.5');
                group.appendChild(wave);
            }
            break;
    }
}

/**
 * Create a unit counter SVG element
 */
function createUnitElement(q, r, unit, unitDef) {
    const { x, y } = axialToPixel(q, r);

    const g = document.createElementNS('http://www.w3.org/2000/svg', 'g');
    g.setAttribute('class', 'unit-group unit-counter');
    g.setAttribute('data-unit-id', unit.id);

    // Unit counter (rectangle with border)
    const rect = document.createElementNS('http://www.w3.org/2000/svg', 'rect');
    rect.setAttribute('x', x - 18);
    rect.setAttribute('y', y - 12);
    rect.setAttribute('width', 36);
    rect.setAttribute('height', 24);
    rect.setAttribute('rx', 3);
    rect.setAttribute('class', `unit unit-${unitDef.side}`);
    g.appendChild(rect);

    // Unit type indicator (small icon at top)
    const typeIcon = document.createElementNS('http://www.w3.org/2000/svg', 'text');
    typeIcon.setAttribute('x', x);
    typeIcon.setAttribute('y', y - 18);
    typeIcon.setAttribute('class', 'unit-type-icon');
    typeIcon.setAttribute('font-size', '8');
    typeIcon.textContent = getUnitTypeIcon(unitDef.type);
    g.appendChild(typeIcon);

    // Unit ID text (centered)
    const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
    text.setAttribute('x', x);
    text.setAttribute('y', y + 4);
    text.setAttribute('class', 'unit-text');
    text.setAttribute('font-size', '11');
    text.textContent = unit.id;
    g.appendChild(text);

    // Strength indicator (larger, at bottom)
    const strength = unitDef[unit.strength === 'full' ? 'full_strength' : 'half_strength'];
    const strengthText = document.createElementNS('http://www.w3.org/2000/svg', 'text');
    strengthText.setAttribute('x', x);
    strengthText.setAttribute('y', y + 20);
    strengthText.setAttribute('class', 'unit-text');
    strengthText.setAttribute('font-size', '9');
    strengthText.setAttribute('font-weight', 'bold');
    strengthText.textContent = strength;
    g.appendChild(strengthText);

    // Movement pips
    addMovementPips(g, x - 16, y - 10, unitDef.movement);

    // Strength indicator pips (show if reduced)
    if (unit.strength === 'half') {
        const halfCircle = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
        halfCircle.setAttribute('cx', x + 14);
        halfCircle.setAttribute('cy', y - 9);
        halfCircle.setAttribute('r', 3);
        halfCircle.setAttribute('class', 'unit-strength-pip');
        halfCircle.setAttribute('opacity', '0.7');
        g.appendChild(halfCircle);
    }

    return g;
}

/**
 * Get a symbol representing unit type
 */
function getUnitTypeIcon(type) {
    const icons = {
        'infantry': '⚔',
        'armor': '▮',
        'cavalry': '🐴',
        'motorized': '⚙',
        'artillery': '💣',
        'airborne': '✈'
    };
    return icons[type] || '●';
}

/**
 * Add movement capability indicator pips
 */
function addMovementPips(group, x, y, movement) {
    const pipCount = Math.min(movement, 3);
    for (let i = 0; i < pipCount; i++) {
        const pip = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
        pip.setAttribute('cx', x + (i * 4));
        pip.setAttribute('cy', y);
        pip.setAttribute('r', 1.5);
        pip.setAttribute('class', 'unit-movement-pip');
        group.appendChild(pip);
    }
}

/**
 * Calculate the bounding box for the map
 */
function calculateMapBounds(hexes) {
    if (!hexes || hexes.length === 0) {
        return { minX: 0, maxX: 1000, minY: 0, maxY: 800 };
    }

    let minX = Infinity, maxX = -Infinity;
    let minY = Infinity, maxY = -Infinity;

    for (const hex of hexes) {
        const { x, y } = axialToPixel(hex.q, hex.r);
        minX = Math.min(minX, x - HEX_SIZE);
        maxX = Math.max(maxX, x + HEX_SIZE);
        minY = Math.min(minY, y - HEX_SIZE);
        maxY = Math.max(maxY, y + HEX_SIZE);
    }

    return {
        minX: minX - 50,
        maxX: maxX + 50,
        minY: minY - 50,
        maxY: maxY + 50
    };
}
