# .sdo, Silly Data Objects

My custom .sdo file format is very simple, essentially just plain text, there's three main sections:
- `Title`, the title of the window.
- `Icon`, the **text** icon of the window, technically allows emojis too but I'll get angry if you use any (/j).
- `Content`, the content of the window.

Each section can span multiple lines if you please, it will stop at the next section and the content will be tirmmed too, so you can space them about however you want. The `Content:` section can have ANYTHING without breaking the parsing, so you can even put `Icon: Imma break you` in it and it will work as expected, unless 'as expected' for you means breaking my parser, asshole.

File names are semi important, their name will mostly be ignored with the exeption of the alphabetical order, so if you want to make something appear first, add `a_` before its filename, or any other letter/number to get it to be ordered however you need.

Silly Data Object files also support comments at the top of the file, for convention please use this format when making .sdo files:
```
<COMMENT>
---

Title: <TITLE>
Icon: <ICON>
Content: <CONTENT, can span multiple lines>
```

# Silly Data Formatter

It's just markdown. *But* I styled it in a way where it will look amazing with my style! One note, code blocks will look like normal text, thus if you wanna escape A LOT of text just wrap it in a code block, easy as that!

> [!NOTE]
> Use SDF sparingly for links and emphasis, remember that this is a simple website, and we dont want fancy headers, I'm too lazy to disable markdown features so please just listen to me on this one.
