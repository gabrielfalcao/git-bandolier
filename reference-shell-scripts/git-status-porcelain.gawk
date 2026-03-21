#!/usr/bin/env gawk -f

BEGIN {
    # print("\x1b[2J\x1b[3J;\x1b[H") >> "/dev/stderr";
    PROCINFO["sorted_in"]="@ind_num_asc"


    delete field_count_by_index;
    delete field_count_by_value;
    delete field_count_by_row;

    delete raw_lines_by_index;
    delete raw_line_rows_by_index;
    delete raw_line_lengths_by_index;

    delete raw_lines_by_value;
    delete raw_line_rows_by_value;
    delete raw_line_lengths_by_value;

    delete raw_lines_by_row;
    delete raw_line_rows_by_row;
    delete raw_line_lengths_by_row;

    delete indexed;
    delete unstaged;
    delete untracked;
    delete field_data_by_field_number;

    raw_line_index=0;
    widest_line_length=0;

}

{
    line_val=$0;
    line_idx=raw_line_index;
    line_len=length(line_val);

    if (line_len > widest_line_length) {
        widest_line_length=line_val;
    }

    delete field_count_by_index;
    delete field_count_by_value;
    delete field_count_by_row;

    delete raw_lines_by_index;
    delete raw_line_rows_by_index;
    delete raw_line_lengths_by_index;

    delete raw_lines_by_value;
    delete raw_line_rows_by_value;
    delete raw_line_lengths_by_value;

    delete raw_lines_by_row;
    delete raw_line_rows_by_row;
    delete raw_line_lengths_by_row;



}

END {
    idx=0
    total=length(pidcounters)
    result="("
    for (keypid in pidcounters) {
        if (idx+1 == total) {
            suffix=")"
        } else {
            suffix="|"
        }

        result=sprintf("%s%s%s", result, keypid, suffix);
        idx++
    }
    printf("    %s\n", result)
}
