import { TrashIcon } from "./icons";

interface Props {
  avatarUrl: string;
  onDelete: () => void;
}

/** The toolbar above a document: its owner's avatar and a delete action. */
export function Toolbar({ avatarUrl, onDelete }: Props) {
  return (
    <div className="toolbar">
      <img className="avatar" src={avatarUrl} />
      <div className="icon-button" onClick={onDelete}>
        <TrashIcon />
      </div>
    </div>
  );
}
