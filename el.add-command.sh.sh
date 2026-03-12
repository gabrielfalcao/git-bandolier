#!/usr/bin/env bash

sed -E "s/${from_name}/${to_name}/g" -i "${rsfile}"
sed -E "s/${from_title}/${to_title}/g" -i "${rsfile}"
sed -E "s,^(\s*)[/][/](\s*)((Command::)?${to_title}),\1\2\3,g" -i src/main.rs
sed -E "s@${from_title}(${from_title}Opt),@${from_title}(${from_title}Opt),${from_title}(${from_titleOpt}),\n${to_title}(${to_titleOpt}),@g" -i src/main.rs
sed -E "s@(Command::${from_title}(op) => op.dispatch()?)@Command::${from_title}(op) => op.dispatch()?,\n    Command::${to_title}(op) => op.dispatch()?,\n@g" -i src/main.rs
sed -z -E 's@((use\s-+workbench::cli::commands::[{][[:space:]\n]+[^}]+Opt,)([[:space:]\n]*))[}][[:space:]\n]*,[[:space:]\n]*[;]@\1, ${to_title}Opt, \n[}];@g' -i src/main.rs
