import DOMPurify from 'dompurify';
import { marked } from 'marked';

const markdown = function(/* @type {string | undefined | null} */ string) {
  if (string)
    return DOMPurify.sanitize(marked.parse(string));
};

export default markdown;
