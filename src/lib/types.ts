export type HexColor = string;

export interface SvgFile {
	path: string;
	size: number;
	content: string;
}

export interface ScanResult {
	source_dir: string;
	preview_svgs: SvgFile[];
	total_svg_count: number;
	non_svg_count: number;
}

export interface ColorMapping {
	original: HexColor;
	replacement: HexColor;
}

export interface ExportResult {
	output_dir: string;
	svgs_processed: number;
	files_copied: number;
}
