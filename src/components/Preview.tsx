import { HTMLAttributes } from "react"
import root from 'react-shadow';

type Props = {
  html: string;
  stylesheets: string[];
} & HTMLAttributes<HTMLDivElement>;

export default function Preview({ html, stylesheets, ...props }: Props) {
  return (
    <root.div {...props}>
      {
        stylesheets.map((s) => (
          <link key={s} rel="stylesheet" href={`/${s}`} />)
        )
      }
      <body dangerouslySetInnerHTML={{__html: html}} style={{ padding: 0 }}></body>
    </root.div>
  )
}
