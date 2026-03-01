(start_tag
  name: (tag_name) @tag)
(end_tag
  name: (tag_name) @tag)
(self_closing_element
  name: (tag_name) @tag)

(attribute
  name: (attribute_name) @attribute)
(attribute
  value: (attribute_value
    (string) @string))
(attribute
  value: (attribute_value
    (unquoted_attribute_value) @string))
(string (escape_sequence) @escape)

(comment) @comment
(entity_reference) @constant

[(text) (raw_text)] @text
