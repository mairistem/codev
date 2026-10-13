import { useId, useRef, useState } from "react";

/**
 * A select-only combobox, as the ARIA Authoring Practices describe it:
 * the button holds focus, `aria-activedescendant` names the highlighted
 * option, and Tab leaves the control after choosing the highlighted size.
 */
export function SizePicker({ sizes, value, onChange }: { sizes: string[]; value: string; onChange: (size: string) => void }) {
  const id = useId();
  const [open, setOpen] = useState(false);
  const [active, setActive] = useState(Math.max(0, sizes.indexOf(value)));
  const combo = useRef<HTMLDivElement>(null);

  function choose(index: number) {
    onChange(sizes[index]);
    setOpen(false);
    combo.current?.focus();
  }

  /** Opens the list with the current size highlighted. */
  function show() {
    setActive(Math.max(0, sizes.indexOf(value)));
    setOpen(true);
  }

  function onKeyDown(event: React.KeyboardEvent) {
    if (!open) {
      if (["ArrowDown", "ArrowUp", "Enter", " "].includes(event.key)) {
        event.preventDefault();
        show();
      }
      return;
    }
    switch (event.key) {
      case "ArrowDown":
        event.preventDefault();
        setActive(Math.min(active + 1, sizes.length - 1));
        break;
      case "ArrowUp":
        event.preventDefault();
        setActive(Math.max(active - 1, 0));
        break;
      case "Home":
        event.preventDefault();
        setActive(0);
        break;
      case "End":
        event.preventDefault();
        setActive(sizes.length - 1);
        break;
      case "Enter":
      case " ":
        event.preventDefault();
        choose(active);
        break;
      case "Escape":
        event.preventDefault();
        setOpen(false);
        break;
      case "Tab":
        onChange(sizes[active]);
        setOpen(false);
        break;
    }
  }

  return (
    <div className="size-picker">
      <span id={`${id}-label`}>Size</span>
      <div
        ref={combo}
        role="combobox"
        tabIndex={0}
        aria-labelledby={`${id}-label`}
        aria-haspopup="listbox"
        aria-controls={`${id}-list`}
        aria-expanded={open}
        aria-activedescendant={open ? `${id}-option-${active}` : undefined}
        onKeyDown={onKeyDown}
        onClick={() => (open ? setOpen(false) : show())}
        onBlur={() => setOpen(false)}
      >
        {value}
      </div>
      <ul role="listbox" id={`${id}-list`} aria-labelledby={`${id}-label`} hidden={!open}>
        {sizes.map((size, index) => (
          <li
            key={size}
            id={`${id}-option-${index}`}
            role="option"
            aria-selected={index === active}
            className={index === active ? "option option--active" : "option"}
            onMouseDown={(event) => event.preventDefault()}
            onClick={() => choose(index)}
          >
            {size}
          </li>
        ))}
      </ul>
    </div>
  );
}
