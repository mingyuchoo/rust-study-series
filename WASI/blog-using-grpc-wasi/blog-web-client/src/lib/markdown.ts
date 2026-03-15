import { Marked } from 'marked';

const marked = new Marked({
	breaks: true,
	gfm: true
});

export function renderMarkdown(content: string): string {
	const html = marked.parse(content);
	if (typeof html !== 'string') {
		return content;
	}
	return html;
}

export function renderExcerpt(content: string, maxLength = 200): string {
	const html = marked.parse(content);
	if (typeof html !== 'string') {
		return content.slice(0, maxLength);
	}
	const text = html.replace(/<[^>]*>/g, '').replace(/\s+/g, ' ').trim();
	if (text.length <= maxLength) {
		return text;
	}
	return text.slice(0, maxLength) + '…';
}
