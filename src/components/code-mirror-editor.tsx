import CodeMirror, { type Extension } from "@uiw/react-codemirror"
import { json } from "@codemirror/lang-json"

type CodeMirrorEditorProps = {
  value: string
  onChange?: (value: string) => void
  extensions?: Extension[]
  height?: string
  readOnly?: boolean
}

export function CodeMirrorEditor({
  value,
  onChange,
  extensions = [json()],
  height = "200px",
  readOnly = false,
}: CodeMirrorEditorProps) {
  return (
    <CodeMirror
      value={value}
      height={height}
      readOnly={readOnly}
      extensions={extensions}
      onChange={onChange}
    />
  )
}
