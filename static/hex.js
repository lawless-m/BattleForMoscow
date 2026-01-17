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
 * Create a unit counter SVG element
 */
function createUnitElement(q, r, unit, unitDef) {
    const { x, y } = axialToPixel(q, r);

    const g = document.createElementNS('http://www.w3.org/2000/svg', 'g');
    g.setAttribute('class', 'unit-group');
    g.setAttribute('data-unit-id', unit.id);

    // Unit counter (rectangle)
    const rect = document.createElementNS('http://www.w3.org/2000/svg', 'rect');
    rect.setAttribute('x', x - 15);
    rect.setAttribute('y', y - 10);
    rect.setAttribute('width', 30);
    rect.setAttribute('height', 20);
    rect.setAttribute('rx', 3);
    rect.setAttribute('class', `unit unit-${unitDef.side}`);
    g.appendChild(rect);

    // Unit ID text
    const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
    text.setAttribute('x', x);
    text.setAttribute('y', y + 5);
    text.setAttribute('class', 'unit-text');
    text.textContent = unit.id;
    g.appendChild(text);

    // Strength indicator
    const strength = unitDef[unit.strength === 'full' ? 'full_strength' : 'half_strength'];
    const strengthText = document.createElementNS('http://www.w3.org/2000/svg', 'text');
    strengthText.setAttribute('x', x);
    strengthText.setAttribute('y', y - 15);
    strengthText.setAttribute('class', 'unit-text');
    strengthText.setAttribute('font-size', '10');
    strengthText.textContent = strength;
    g.appendChild(strengthText);

    return g;
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
