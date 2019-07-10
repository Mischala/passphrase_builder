with open('words.txt') as inputfile:
    num_lines = len(inputfile.readlines())


with open('words.txt') as inputfile:
    with open('word_list.rs', "w+") as outputfile:
        outputfile.writelines([
            r"pub fn get_word(num: usize) -> &'static str {" + "\n",
            "    let word_list: &[&'static str; " + str(num_lines) + "] = &["+ "\n",
            ])

        for count, line in enumerate(inputfile):
            outputfile.write(r'        "' + line.rstrip() + "\",\n")

        outputfile.writelines([
            r"    ];" + "\n",
            "\n",
            r"return word_list[num];" + "\n",
            r"}" +"\n"
        ])

with open('words.txt') as inputfile:
    print("words.txt is " + str(num_lines) + " lines long.")