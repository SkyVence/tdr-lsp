(start_tag) @indent.begin
(end_tag) @indent.end
(self_closing_element) @indent.align
(raw_object_element start_tag: (start_tag)) @indent.begin
(raw_object_element end_tag: (end_tag)) @indent.end
(text) @indent.auto
