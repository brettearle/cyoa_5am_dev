# REQUIREMENTS

## Terminal-Based Choose Your Own Adventure Game

### ✅ Overview

This is a language-agnostic terminal application that loads a branching narrative from a JSON file and allows users to navigate it via text-based input. The game presents story sections with choices, updates state based on input, and concludes with one of several possible endings.

---
### 🎓 Learning Outcomes

1. **Understand control flow** using conditionals, loops, and input validation.
2. **Work with structured data** (JSON) for dynamic content loading.
3. **Parse and traverse nested data structures** such as maps/dictionaries and arrays.
4. **Build a text-based CLI interface** using standard input/output.
5. **Separate content from logic** by designing with external story files.
6. **Handle user input safely** and provide feedback for invalid choices.
7. **Implement a finite state navigation system** (moving between story nodes).
8. **Write portable code** without external libraries.
9. **Structure code for clarity and maintainability**, with modularity and comments.
10. **(Optional)** Implement basic CLI flags like `--help` for usability.

---
## 1. 🧩 Functional Requirements

### 📘 Story Handling

* [ ] Load story content from a JSON file (`story.json`).
* [ ] Parse the following top-level keys from the JSON:

  * `"start"`: ID of the starting section.
  * `"sections"`: All intermediate story nodes.
  * `"endings"`: Terminal story conclusions.

### 🧭 Navigation & Choices

* [ ] Display the `text` of the current section.
* [ ] List available choices with their labels (`a`, `b`, `c`, etc.).
* [ ] Prompt the user to input a valid choice.
* [ ] Validate the input and re-prompt on invalid choices.
* [ ] Update the story state to the next section based on input.
* [ ] Repeat the loop until an ending is reached.

### 🎯 Endings

* [ ] Detect when a node is a terminal node (i.e., `end_1`, `end_2`, etc.).
* [ ] Display the appropriate ending from the `endings` section.
* [ ] End the game gracefully after showing the conclusion.

---

## 2. 🛠️ Non-Functional Requirements

* [ ] **Language-Agnostic**: No dependencies outside of the standard library.
* [ ] **Portable**: Must work on macOS, Linux, and Windows terminals. 
* [ ] **Simple I/O**: Only use `stdin` and `stdout` for interaction.
* [ ] **Self-contained**: Story data should be easily replaceable without code changes.
* [ ] **Minimal**: Codebase must be <300 lines (excluding comments).
* [ ] **Readable**: Use clear variable names and inline comments.
* [ ] **Optional**: Support a `--help` or `-h` flag to show basic instructions.

---

## 3. 📂 File Structure

```
/project-root
├── src # Main source files (.py, .rs, .js, etc.)
├── story.json # Story content file
├── README.md # Game description and how-to-run
└── REQUIREMENTS.md # This file
```

---

## 4. 📋 User Flow Example

```bash
$ cyoa_app

You wake up in a misty forest. A chill runs down your spine.

What do you do?
[a] Follow a distant howl
[b] Climb a nearby tree
[c] Stay still and listen

> b

From the tree, you see a figure darting through the trees.

What do you do?
[a] Climb down and chase it
[b] Hide and watch

> a

...

You escape the forest and find refuge. Your story becomes legend.

[The End]
```

---

## 5. 🧪 Stretch Goals (Optional)

* [ ] Add colorized output (e.g., headings in bold or colored).
* [ ] Add a flag to randomize choices or shuffle paths for replayability.
* [ ] Add a logging/debug mode to print state transitions.
