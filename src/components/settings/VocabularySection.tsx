import { useState } from "react";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Plus, X } from "lucide-react";

interface VocabularySectionProps {
  vocabulary: string[];
  onVocabularyChange: (words: string[]) => void;
}

export default function VocabularySection({
  vocabulary,
  onVocabularyChange,
}: VocabularySectionProps) {
  const [newWord, setNewWord] = useState("");

  const addWord = () => {
    const word = newWord.trim();
    if (!word || vocabulary.includes(word)) return;
    onVocabularyChange([...vocabulary, word]);
    setNewWord("");
  };

  const removeWord = (word: string) => {
    onVocabularyChange(vocabulary.filter((w) => w !== word));
  };

  const handleImport = () => {
    const input = document.createElement("input");
    input.type = "file";
    input.accept = ".txt";
    input.onchange = async (e) => {
      const file = (e.target as HTMLInputElement).files?.[0];
      if (!file) return;
      const text = await file.text();
      const words = text
        .split("\n")
        .map((w) => w.trim())
        .filter((w) => w && !vocabulary.includes(w));
      if (words.length) {
        onVocabularyChange([...vocabulary, ...words]);
      }
    };
    input.click();
  };

  const handleExport = () => {
    const blob = new Blob([vocabulary.join("\n")], { type: "text/plain" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "vocabulary.txt";
    a.click();
    URL.revokeObjectURL(url);
  };

  return (
    <section>
      <h2 className="text-[length:var(--font-size-heading-2)] font-semibold text-text-primary">
        Custom Vocabulary
      </h2>
      <p className="mt-1 text-[length:var(--font-size-caption)] text-text-secondary">
        Words and phrases to improve recognition accuracy
      </p>

      <div className="mt-4 space-y-4">
        {/* Add word */}
        <div className="flex items-center gap-2">
          <Input
            placeholder="Add a word or phrase..."
            value={newWord}
            onChange={(e) => setNewWord(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && addWord()}
            className="flex-1"
          />
          <Button size="sm" onClick={addWord} disabled={!newWord.trim()}>
            <Plus size={14} className="mr-1" /> Add
          </Button>
        </div>

        {/* Word list */}
        {vocabulary.length > 0 && (
          <div className="flex flex-wrap gap-2">
            {vocabulary.map((word) => (
              <span
                key={word}
                className="flex items-center gap-1 rounded-full border border-border-subtle bg-bg-elevated px-2.5 py-1 text-[length:var(--font-size-caption)] text-text-primary"
              >
                {word}
                <button
                  type="button"
                  onClick={() => removeWord(word)}
                  className="text-text-tertiary hover:text-accent-error"
                >
                  <X size={12} />
                </button>
              </span>
            ))}
          </div>
        )}

        {/* Import/Export */}
        <div className="flex gap-2">
          <Button variant="outline" size="sm" onClick={handleImport}>
            Import
          </Button>
          <Button
            variant="outline"
            size="sm"
            onClick={handleExport}
            disabled={vocabulary.length === 0}
          >
            Export
          </Button>
        </div>
      </div>
    </section>
  );
}
