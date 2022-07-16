import { FC, useCallback } from "react";

export interface CopiedTextItemProps {
  text: string;
  onCopy: (text: string) => void;
}

export const CopiedTextItem: FC<CopiedTextItemProps> = (props) => {
  const { text, onCopy } = props;

  const handleCopy = useCallback(() => {
    onCopy?.(text);
  }, [text]);

  return (
    <div
      className="box-border w-96 rounded-md bg-purple-600 bg-opacity-20 p-4"
      onClick={handleCopy}
    >
      <div className="font-sans text-lg text-purple-700 text-opacity-100 border-purple-500 border-4 rounded-md p-4">
        {text}
      </div>

      <button className="bg-purple-500 ring-4 ring-purple-400 text-white rounded-md w-full py-2 mt-4 hover:bg-opacity-90 active:bg-purple-500">
        Copy
      </button>
    </div>
  );
};
