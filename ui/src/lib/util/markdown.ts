import markdownit from "markdown-it";
import { full as emoji } from "markdown-it-emoji";
import hljs from "highlight.js";

const md = markdownit({
    html: false,
    xhtmlOut: false,
    breaks: false,
    langPrefix: "hljs-",
    linkify: true,
    typographer: true,
    quotes: "“”‘’",
    highlight: (str, lang) => {
        if (lang && hljs.getLanguage(lang)) {
            try {
                return hljs.highlight(str, { language: lang }).value;
            } catch (__) {}
        }

        return "";
    },
}).use(emoji);

export const markdownInline = (str: string): string => md.renderInline(str);
export const markdown = (str: string): string => md.render(str);
export const removeBase = (target: string, base: string) => target.replace(base, "");
