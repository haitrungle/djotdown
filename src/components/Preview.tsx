import { HTMLAttributes } from "react"
import root from 'react-shadow';

type Props = { html: string } & HTMLAttributes<HTMLDivElement>;

export default function Preview({ html, ...props }: Props) {
  const markup = substringBetween(html, '<body>', '</body>')
  return (
    <root.div {...props}>
      <link rel="stylesheet" href="/bamboo.css" />
      <body dangerouslySetInnerHTML={{__html: markup}} style={{ padding: 0 }}></body>
    </root.div>
  )
}

function substringBetween(str: string, a: string, b: string) {
  const start = str.indexOf(a) + a.length
  return str.substring(start, str.indexOf(b, start))
}