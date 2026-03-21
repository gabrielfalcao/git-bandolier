;; (let ( (label-regexp "\4") (porcelain-status-regexp \2) (label (string-trim (replace-regexp-in-string "[^a-z]+" "_" \2) "_- \r\t\n")) ) (format "[\x22%s\x22]=\x22%s\x22" label porcelain-status-regexp) )


(defun regex-group-from-string-sequence (parts regex-syntax)
 (unless (sequencep parts)
   (error "regex-group-from-string-sequence arg `PARTS' is a %s instead of sequence: %S"
          (type-of parts) parts))

 (let ((total (length parts))
       (seq-ty (type-of parts)))
   (seq-do-indexed (lambda (value index)
                     (let ((element-location (format "element %d of %d in %s `PARTS'"
                                                     (+ index 1)
                                                     total
                                                     seq-ty)))
                       (unless (stringp value)
                         (error "regex-group-from-string-sequence %s is a %s instead of string: %S"
                            element-location
                            (type-of value)
                            value)))
                 parts)
          (type-of parts) parts))



 )


(let* (
     (regex-syntax :read)
     (symbol-regex-pattern "[a-zA-Z_-]+[a-zA-Z0-9_-]*")

     (nonspc-regex-pattern "[^a-zA-Z0-9_-\x22\x27]+")
     (space-regexp-pattern "[[:space:]]+")

     (interspace-pattern-sequence (list nonspc-regex-pattern space-regexp-pattern nonspc-regex-pattern))
     (outerspace-pattern-sequence (list space-regexp-pattern nonspc-regex-pattern space-regexp-pattern))

     (interspace-group (format "(%s)" (list nonspc-regex-pattern space-regexp-pattern nonspc-regex-pattern))
     (outerspace-group (list space-regexp-pattern nonspc-regex-pattern space-regexp-pattern))




)
