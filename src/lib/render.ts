import * as djot from '@djot/djot'
import { Warning } from '@djot/djot/types/options'
import { CodeBlock, Visitor } from '@djot/djot/types/ast'

import { stylesheets } from '@/config.json'

interface RenderOptions {
  sourcePositions?: boolean
  warnHandler?: (w: Warning) => void
  overrides?: Visitor<djot.HTMLRenderer, string>
  fullHtml?: boolean
}

export default function renderHtmlFromDjot(
  content: string,
  { sourcePositions, warnHandler, overrides, fullHtml }: RenderOptions
) {
    const ast = djot.parse(content, {
      sourcePositions: sourcePositions || false,
      warn: warnHandler,
    })
    const markup = djot.renderHTML(ast, {
      warn: warnHandler,
      overrides: overrides,
    })
    return fullHtml ? toFullHtml(markup) : markup
}

/*
Same as `renderHtmlFromDjot`, but transforms Djot code block
to add HTML render result
*/
export function renderHtmlWithDjotExample(
  content: string,
  warnHandler: (w: Warning) => void,
) {
  const options = {
    sourcePositions: true,
    warnHandler,
    overrides: {
      code_block(node: CodeBlock, renderer: djot.HTMLRenderer) {
        const code = renderer.renderAstNodeDefault(node)
        if (node.lang && node.lang.toLowerCase() === 'djot') {
          const html = renderHtmlFromDjot(node.text, {})
          const escapedHtml = new Option(html).innerHTML

          return (
            `<div class="djot-example">
              ${code}
              <pre><code>${escapedHtml}</code></pre>
            </div>`
          )
        }
        return code
      },
    },
    fullHtml: false,
  }

  return renderHtmlFromDjot(
    content,
    options,
  )
}

export function toFullHtml(markup: string) {
  return `<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Djot</title>
    ${
      stylesheets
        .map((s) => `<link rel="stylesheet" href="/${s}">`)
        .join('\n')
    }
  </head>
  <body>
    ${markup}
  </body>
</html>`
}