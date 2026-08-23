const list = document.querySelector("#task-list");
const count = document.querySelector("#task-count");
const error = document.querySelector("#task-error");

function render(tasks, summary) {
  list.replaceChildren();
  for (const task of tasks) {
    const item = document.createElement("li");
    item.className = task.done ? "task done" : "task";

    const marker = document.createElement("span");
    marker.className = "task-marker";
    marker.textContent = task.done ? "✓" : "•";
    marker.setAttribute("aria-hidden", "true");

    const title = document.createElement("span");
    title.className = "task-title";
    title.textContent = task.title;

    item.append(marker, title);
    list.append(item);
  }
  count.textContent = `${summary.completed} of ${summary.total} complete`;
}

async function loadTasks() {
  try {
    const response = await fetch("/api/tasks", {
      headers: { Accept: "application/json" },
    });
    if (!response.ok) {
      throw new Error(`API returned ${response.status}`);
    }
    const payload = await response.json();
    render(payload.tasks, payload.summary ?? {
      total: payload.tasks.length,
      completed: payload.tasks.filter((task) => task.done).length,
    });
  } catch (cause) {
    count.textContent = "Unavailable";
    error.hidden = false;
    error.textContent = `Could not load tasks: ${cause.message}`;
  }
}

loadTasks();
