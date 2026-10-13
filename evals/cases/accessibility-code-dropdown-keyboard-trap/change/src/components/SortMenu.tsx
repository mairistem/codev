import { useEffect, useRef, useState } from "react";

export const SORT_OPTIONS = ["Newest", "Oldest", "Price: low to high", "Price: high to low"];

/** A menu button that picks the sort order of the product list. */
export function SortMenu({ value, onChange }: { value: string; onChange: (value: string) => void }) {
  const [open, setOpen] = useState(false);
  const button = useRef<HTMLButtonElement>(null);
  const items = useRef<(HTMLButtonElement | null)[]>([]);

  useEffect(() => {
    if (open) items.current[0]?.focus();
  }, [open]);

  function choose(option: string) {
    onChange(option);
    setOpen(false);
    button.current?.focus();
  }

  function onKeyDown(event: React.KeyboardEvent) {
    const index = items.current.findIndex((item) => item === document.activeElement);
    if (event.key === "ArrowDown" || event.key === "Tab") {
      event.preventDefault();
      items.current[(index + 1) % SORT_OPTIONS.length]?.focus();
    } else if (event.key === "ArrowUp") {
      event.preventDefault();
      items.current[(index - 1 + SORT_OPTIONS.length) % SORT_OPTIONS.length]?.focus();
    }
  }

  return (
    <div className="sort-menu">
      <button ref={button} type="button" aria-haspopup="menu" aria-expanded="false" onClick={() => setOpen(!open)}>
        Sort: {value}
      </button>
      {open && (
        <ul role="menu" aria-label="Sort products" onKeyDown={onKeyDown}>
          {SORT_OPTIONS.map((option, i) => (
            <li key={option} role="none">
              <button
                ref={(element) => (items.current[i] = element)}
                type="button"
                role="menuitemradio"
                aria-checked={option === value}
                onClick={() => choose(option)}
              >
                {option}
              </button>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
