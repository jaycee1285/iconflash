<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { open } from '@tauri-apps/plugin-dialog';
	import { extractColorsFromMultiple, replaceColorInSvg } from '$lib/colors';
	import type { ScanResult, ColorMapping, ExportResult } from '$lib/types';

	let scanResult = $state<ScanResult | null>(null);
	let extractedColors = $state<string[]>([]);
	let colorMappings = $state<ColorMapping[]>([]);
	let themeName = $state('');
	let isScanning = $state(false);
	let isExporting = $state(false);
	let error = $state<string | null>(null);
	let exportSuccess = $state<string | null>(null);

	let originalSvg = $derived(scanResult?.preview_svgs[0]?.content ?? '');

	let previewSvg = $derived.by(() => {
		if (!originalSvg) return '';
		let svg = originalSvg;
		for (const mapping of colorMappings) {
			if (mapping.original !== mapping.replacement && mapping.replacement.length === 7) {
				svg = replaceColorInSvg(svg, mapping.original, mapping.replacement);
			}
		}
		return svg;
	});

	let hasReplacements = $derived(
		colorMappings.some(
			(m) => m.original !== m.replacement && /^#[0-9a-fA-F]{6}$/.test(m.replacement)
		)
	);

	let canExport = $derived(hasReplacements && themeName.trim().length > 0 && !isExporting);

	async function selectDirectory() {
		error = null;
		exportSuccess = null;
		const selected = await open({
			directory: true,
			recursive: true,
			title: 'Select Icon Theme Directory'
		});
		if (!selected) return;

		isScanning = true;
		try {
			scanResult = await invoke<ScanResult>('scan_directory', { path: selected });
			if (scanResult.preview_svgs.length === 0) {
				error = 'No SVG files found in this directory.';
				scanResult = null;
				return;
			}
			extractedColors = extractColorsFromMultiple(scanResult.preview_svgs);
			colorMappings = extractedColors.map((c) => ({ original: c, replacement: c }));
		} catch (e) {
			error = String(e);
			scanResult = null;
		} finally {
			isScanning = false;
		}
	}

	async function doExport() {
		if (!scanResult || !canExport) return;
		isExporting = true;
		error = null;
		exportSuccess = null;

		const mappings: [string, string][] = colorMappings
			.filter((m) => m.original !== m.replacement && /^#[0-9a-fA-F]{6}$/.test(m.replacement))
			.map((m) => [m.original, m.replacement]);

		try {
			const result = await invoke<ExportResult>('export_theme', {
				sourceDir: scanResult.source_dir,
				themeName: themeName.trim(),
				colorMappings: mappings
			});
			exportSuccess = `Exported to ${result.output_dir} (${result.svgs_processed} SVGs, ${result.files_copied} other files)`;
		} catch (e) {
			error = String(e);
		} finally {
			isExporting = false;
		}
	}

	function updateReplacement(index: number, value: string) {
		const cleaned = value.startsWith('#') ? value : `#${value}`;
		colorMappings[index] = { ...colorMappings[index], replacement: cleaned.toLowerCase() };
	}
</script>

<div class="flex h-screen flex-col bg-surface text-text">
	<!-- Top bar -->
	<header class="flex items-center gap-4 border-b border-border bg-surface-alt px-4 py-3">
		<button
			onclick={selectDirectory}
			disabled={isScanning}
			class="rounded-md bg-accent px-4 py-2 text-sm font-medium text-surface transition-colors hover:bg-accent-hover disabled:opacity-50"
		>
			{isScanning ? 'Scanning...' : 'Select Directory'}
		</button>

		{#if scanResult}
			<span class="text-sm text-text-dim">
				{scanResult.source_dir}
			</span>
			<span class="text-sm text-text-dim">
				({scanResult.total_svg_count} SVGs, {scanResult.non_svg_count} other files)
			</span>
		{:else if !isScanning}
			<span class="text-sm text-text-dim">Choose an icon theme directory to get started</span>
		{/if}
	</header>

	<!-- Error / Success messages -->
	{#if error}
		<div class="border-b border-error/30 bg-error/10 px-4 py-2 text-sm text-error">
			{error}
		</div>
	{/if}
	{#if exportSuccess}
		<div class="border-b border-success/30 bg-success/10 px-4 py-2 text-sm text-success">
			{exportSuccess}
		</div>
	{/if}

	<!-- Main content -->
	{#if scanResult}
		<div class="flex min-h-0 flex-1">
			<!-- Left panel: Original -->
			<div class="flex w-1/2 flex-col border-r border-border p-4">
				<h2 class="mb-3 text-sm font-semibold uppercase tracking-wider text-text-dim">
					Original
				</h2>

				<!-- SVG Preview -->
				<div
					class="mb-4 flex aspect-square max-h-64 items-center justify-center overflow-hidden rounded-lg bg-surface-alt p-4 [&>svg]:h-full [&>svg]:w-full"
				>
					{@html originalSvg}
				</div>

				<!-- Color swatches -->
				<div class="flex flex-wrap gap-3">
					{#each extractedColors as color}
						<div class="flex items-center gap-2">
							<div
								class="h-8 w-8 rounded border border-border"
								style="background-color: {color}"
							></div>
							<span class="font-mono text-sm text-text-dim">{color}</span>
						</div>
					{/each}
				</div>
			</div>

			<!-- Right panel: Replacement -->
			<div class="flex w-1/2 flex-col p-4">
				<h2 class="mb-3 text-sm font-semibold uppercase tracking-wider text-text-dim">
					Replacement
				</h2>

				<!-- SVG Preview -->
				<div
					class="mb-4 flex aspect-square max-h-64 items-center justify-center overflow-hidden rounded-lg bg-surface-alt p-4 [&>svg]:h-full [&>svg]:w-full"
				>
					{#if hasReplacements}
						{@html previewSvg}
					{:else}
						<span class="text-sm text-text-dim">Enter replacement colors below</span>
					{/if}
				</div>

				<!-- Color inputs -->
				<div class="flex flex-wrap gap-3">
					{#each colorMappings as mapping, i}
						<div class="flex items-center gap-2">
							<div
								class="h-8 w-8 rounded border border-border"
								style="background-color: {/^#[0-9a-fA-F]{6}$/.test(mapping.replacement) ? mapping.replacement : mapping.original}"
							></div>
							<input
								type="text"
								value={mapping.replacement}
								oninput={(e) => updateReplacement(i, e.currentTarget.value)}
								placeholder={mapping.original}
								maxlength={7}
								class="w-24 rounded border border-border bg-surface px-2 py-1 font-mono text-sm text-text focus:border-accent focus:outline-none"
							/>
						</div>
					{/each}
				</div>
			</div>
		</div>

		<!-- Export bar -->
		<footer class="flex items-center gap-4 border-t border-border bg-surface-alt px-4 py-3">
			<label class="flex items-center gap-2 text-sm text-text-dim">
				Theme name:
				<input
					type="text"
					bind:value={themeName}
					placeholder="my-icon-theme"
					class="w-48 rounded border border-border bg-surface px-3 py-1.5 text-sm text-text focus:border-accent focus:outline-none"
				/>
			</label>
			<button
				onclick={doExport}
				disabled={!canExport}
				class="rounded-md bg-accent px-6 py-1.5 text-sm font-medium text-surface transition-colors hover:bg-accent-hover disabled:opacity-50"
			>
				{isExporting ? 'Exporting...' : 'Export'}
			</button>
			<span class="text-xs text-text-dim">
				Saves to ~/.local/share/icons/
			</span>
		</footer>
	{:else if !isScanning}
		<!-- Empty state -->
		<div class="flex flex-1 items-center justify-center">
			<div class="text-center">
				<p class="mb-2 text-lg text-text-dim">No icon theme loaded</p>
				<p class="text-sm text-text-dim">
					Click "Select Directory" to choose an SVG icon theme folder
				</p>
			</div>
		</div>
	{:else}
		<!-- Scanning state -->
		<div class="flex flex-1 items-center justify-center">
			<p class="text-text-dim">Scanning directory for SVG icons...</p>
		</div>
	{/if}
</div>
