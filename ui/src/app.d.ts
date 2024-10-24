// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces

declare global {
	namespace App {
		interface Error {
			code: string;
			id: string;
		}
		// interface Locals {}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}
}

declare module "@editorjs/editorjs" {
	export default class EditorJS {
		constructor(options: any);
		destroy(): Promise<void>;
		save(): Promise<any>;
	}
}

declare module "@editorjs/header";
declare module "@editorjs/paragraph";

export {};
