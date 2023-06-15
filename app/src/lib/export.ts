import { saveAs } from 'file-saver';

export const exportJson = (filename: string, content: string) => {
  const blob = new Blob([content], { type: "application/json;charset=utf-8" });
  saveAs(blob, `${filename}.json`, { autoBom: true });
};
