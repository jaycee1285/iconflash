import type { HexColor, SvgFile } from './types';

function normalizeHex(hex: string): HexColor {
	const lower = hex.toLowerCase();
	if (lower.length === 4) {
		const r = lower[1],
			g = lower[2],
			b = lower[3];
		return `#${r}${r}${g}${g}${b}${b}`;
	}
	return lower;
}

function luminance(hex: HexColor): number {
	const r = parseInt(hex.slice(1, 3), 16);
	const g = parseInt(hex.slice(3, 5), 16);
	const b = parseInt(hex.slice(5, 7), 16);
	return 0.299 * r + 0.587 * g + 0.114 * b;
}

export function extractColors(svgContent: string): HexColor[] {
	// Strip Inkscape/Sodipodi metadata to avoid false-positive colors
	const cleaned = svgContent
		.replace(/<sodipodi:namedview[^]*?\/>/g, '')
		.replace(/<sodipodi:namedview[^]*?<\/sodipodi:namedview>/g, '')
		.replace(/<metadata[^]*?<\/metadata>/g, '');

	// Match hex colors: #RRGGBB or #RGB (not followed by another hex digit)
	const hexRegex = /#([0-9a-fA-F]{6})(?![0-9a-fA-F])|#([0-9a-fA-F]{3})(?![0-9a-fA-F])/g;
	const colorSet = new Set<string>();

	for (const match of cleaned.matchAll(hexRegex)) {
		colorSet.add(normalizeHex(match[0]));
	}

	return Array.from(colorSet).sort((a, b) => luminance(a) - luminance(b));
}

export function extractColorsFromMultiple(svgs: SvgFile[]): HexColor[] {
	const allColors = new Set<string>();
	for (const svg of svgs) {
		for (const color of extractColors(svg.content)) {
			allColors.add(color);
		}
	}
	return Array.from(allColors).sort((a, b) => luminance(a) - luminance(b));
}

function toShortHex(hex: HexColor): string | null {
	if (hex[1] === hex[2] && hex[3] === hex[4] && hex[5] === hex[6]) {
		return `#${hex[1]}${hex[3]}${hex[5]}`;
	}
	return null;
}

function replaceAllInsensitive(str: string, search: string, replacement: string): string {
	const escaped = search.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
	const pattern = new RegExp(`${escaped}(?![0-9a-fA-F])`, 'gi');
	return str.replace(pattern, replacement);
}

export function replaceColorInSvg(
	svg: string,
	originalHex: HexColor,
	replacementHex: HexColor
): string {
	// Replace 6-char form first (case-insensitive)
	let result = replaceAllInsensitive(svg, originalHex, replacementHex);

	// If a 3-char short form exists, replace that too
	const shortOriginal = toShortHex(originalHex);
	if (shortOriginal) {
		const shortReplacement = toShortHex(replacementHex) ?? replacementHex;
		result = replaceAllInsensitive(result, shortOriginal, shortReplacement);
	}

	return result;
}
