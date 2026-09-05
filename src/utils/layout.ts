// Determine the page size and use the correct layout

export enum LayoutMode {
    WIDE = 'WIDE',
    MOBILE_V = 'MOBILE (VERTICAL)',
    MOBILE_H = 'MOBILE (HORIZONTAL)',
    INVALID = 'INVALID'
}

export let currentLayout: LayoutMode = LayoutMode.WIDE;
const layoutWide = document.getElementById('layout-wide')!;
const layoutMobileV = document.getElementById('layout-mobile-v')!;
const layoutMobileH = document.getElementById('layout-mobile-h')!;
const layoutInvalid = document.getElementById('layout-invalid')!;

export function updateLayout(): void {
    const w = window.innerWidth;
    const h = window.innerHeight;
    const aspect = w / h;

    // Determine the new layout
    if (w < 332 || h < 320) currentLayout = LayoutMode.INVALID;  // Unrealistically small/unusual screens
    else if (w >= 900) currentLayout = LayoutMode.WIDE;          // Wide Mode: Large, square-ish screens
    else if (aspect >= 1.2) currentLayout = LayoutMode.MOBILE_H; // Mobile Horizontal: Wide, short screens
    else currentLayout = LayoutMode.MOBILE_V;                    // Mobile Vertical: Tall, narrow screens

    // Update which <div> is visible
    layoutWide.style.display = currentLayout == LayoutMode.WIDE ? 'block' : 'none';
    layoutMobileV.style.display = currentLayout == LayoutMode.MOBILE_V ? 'block' : 'none';
    layoutMobileH.style.display = currentLayout == LayoutMode.MOBILE_H ? 'block' : 'none';
    layoutInvalid.style.display = currentLayout == LayoutMode.INVALID ? 'block' : 'none';
    console.log(`Page resized or reloaded. New size is ${w} wide and ${h} tall (aspect ${aspect.toFixed(3)}). Layout updated to ${currentLayout}`);
}