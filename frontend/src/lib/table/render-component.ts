import type { Component, ComponentProps, Snippet } from "svelte";

/**
 * A helper class to make it easy to identify Svelte components in
 * `columnDef.cell` and `columnDef.header` properties.
 */
export class RenderComponentConfig<TComponent extends Component> {
	constructor(
		public component: TComponent,
		public props: ComponentProps<TComponent> | Record<string, never> = {},
	) {}
}

/**
 * A helper class to make it easy identify Svelte Snippets in `columnDef.cell`
 * and `columnDef.header` properties.
 */
export class RenderSnippetConfig<TProps> {
	constructor(
		public snippet: Snippet<[TProps]>,
		public params: TProps,
	) {}
}

/**
 * A helper function to help create cells from Svelte components through ColumnDef's `cell` and `header` properties.
 *
 * This is only to be used with Svelte Components - use `renderSnippet` for Svelte Snippets.
 *
 * @param component A Svelte component
 * @param props The props to pass to `component`
 * @returns A `RenderComponentConfig` object that helps svelte-table know how to render the header/cell component.
 * @example
 * ```ts
 * // +page.svelte
 * const defaultColumns = [
 *   columnHelper.accessor('name', {
 *     header: header => renderComponent(SortHeader, { label: 'Name', header }),
 *   }),
 *   columnHelper.accessor('state', {
 *     header: header => renderComponent(SortHeader, { label: 'State', header }),
 *   }),
 * ]
 * ```
 * @see {@link https://tanstack.com/table/latest/docs/guide/column-defs}
 */
export const renderComponent = <
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	TComponent extends Component<any>,
	TProps extends ComponentProps<TComponent>,
>(
	component: TComponent,
	props: TProps,
) => new RenderComponentConfig(component, props);

/**
 * A helper function to help create cells from Svelte Snippets through ColumnDef's `cell` and `header` properties.
 *
 * The snippet must only take one parameter.
 *
 * This is only to be used with Snippets - use `renderComponent` for Svelte Components.
 *
 * @param snippet
 * @param params
 * @returns
 * @example
 * ```ts
 * // +page.svelte
 * const defaultColumns = [
 *   columnHelper.accessor('name', {
 *     cell: cell => renderSnippet(nameSnippet, { name: cell.row.name }),
 *   }),
 *   columnHelper.accessor('state', {
 *     cell: cell => renderSnippet(stateSnippet, { state: cell.row.state }),
 *   }),
 * ]
 * ```
 * @see {@link https://tanstack.com/table/latest/docs/guide/column-defs}
 */
export const renderSnippet = <TProps>(snippet: Snippet<[TProps]>, params: TProps) =>
	new RenderSnippetConfig(snippet, params);
