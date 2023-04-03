import DOMPurify from 'dompurify';
import { marked } from 'marked';

const markdown = function(str: string | undefined | null) {
  if (str)
    return DOMPurify.sanitize(marked.parse(str));
};

export default markdown;
