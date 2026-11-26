export interface Task {
  id: number;
  title: string;
  done: boolean;
}

const tasks: Task[] = [];

export function create(title: string): Task {
  if (title.trim() === "") {
    throw new RangeError("a task needs a title");
  }
  const task = { id: tasks.length + 1, title, done: false };
  tasks.push(task);
  return task;
}

export function all(): Task[] {
  return [...tasks];
}
