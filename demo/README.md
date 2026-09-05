# The browser demo

`accent-proust` compiled to WebAssembly, with a source pane, a rendered pane,
and a stylesheet you can swap underneath the output.

```sh
./scripts/serve-demo.sh          # builds the package, serves on 8000
```

Then open <http://127.0.0.1:8000/demo/>. A static server is required: the demo
is an ES module and `init()` fetches the `.wasm`, and a browser refuses both
over `file:`.

## What it is for

Two things, in this order.

**Verification.** It exercises all four entry points -- `renderHtml`,
`validate`, `transform` and `format` -- in a real browser, on whatever you
type. The Node tests in `crates/accent-proust-wasm/tests/` prove the bindings
work; this proves they work where they are meant to run.

**A starting point.** The engine has no opinion about markup or styling: it
emits plain HTML, so a design system is a stylesheet swap and nothing more.
`THEMES` in `demo.js` is a list of `{ id, href, wrap, pad }`, and adding one is
adding an entry. It ships with the U.S. Web Design System and with nothing at
all, which are the two ends of that range.

## How the output is isolated

The rendered document goes into an iframe rather than into the page. A full
design system is half a megabyte of CSS that styles `h1` and `button`, so
loading it into the demo would restyle the demo's own chrome -- and the
`usa-prose` wrapper USWDS expects around content it did not author only makes
sense on a document of its own.

The iframe is `sandbox="allow-same-origin"`, which blocks scripts. That is
belt and braces: Markdoc runs with HTML disabled, so a `<script>` in the source
is text by the time it reaches here. A bare `sandbox` would be stricter still,
and does not work -- it gives the frame an opaque origin and Chrome renders
`srcdoc` as a blank page.

## Where plain HTML meets a design system

The demo is worth running once for this alone, because it is the thing the
engine cannot tell you: a design system styles the elements it expects an
author to write, and a renderer emits the elements the language defines. Those
two sets are close, and they are not the same set.

The code block is the first place they part, and it is instructive precisely
because nothing is wrong.

A fence renders as a bare `<pre data-language="rust">` with **no `<code>`
inside**. That is upstream's own shape, not a liberty this port takes --
`reference/src/schema.ts:53` is `render: 'pre'`, and
`reference/src/ast/node.test.ts:105` asserts `new Tag('pre', { 'data-language':
'ruby' }, ['test'])`.

USWDS gives that element exactly one rule, the normalize idiom
`code,kbd,pre,samp { font-family: monospace,monospace; font-size: 1em }`. The
doubled `monospace` is a deliberate hack that stops browsers shrinking code to
about 13px, and it works. Measured inside the preview frame:

| | computed size | the same string, rendered |
|---|---|---|
| prose text | 16.96px, Source Sans Pro Web | 156.6px wide |
| `<pre>` | 16.96px, generic monospace | 214.3px wide, **1.37x** |

So the code is not set larger. It is set in a face whose glyphs are 37% wider
with a taller x-height, at an identical size, and reads as larger. What USWDS
has not done is give `<pre>` a typeface of its own, because it expects a code
block to carry its own classes; `usa-prose` covers what normalize covers and
stops there.

**The lesson for anything built on this:** budget for a small element mapping
per design system. Not a port, not a fork of the renderer -- a handful of rules
that hand the design system's own tokens to the elements the engine emits. The
demo deliberately does not include one, so that the seam stays visible rather
than being papered over by the thing that is supposed to demonstrate it.

`data-language` is on the element for exactly this reason: it is the hook a
highlighter or a design system's code component attaches to.

## What it does not do

The demo passes no schema configuration, so the tags are Markdoc's built-ins
only and the sample's `{% callout %}` reports `tag-undefined` -- which is the
point of that part of the sample.

The bindings themselves do take one now: `new Config({ tags: ... })` and the
same stages as methods on it. The demo deliberately does not, because the
undefined tag is the clearest way to show what the diagnostics are for.
