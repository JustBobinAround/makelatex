use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Not enough arguments");
    }else{
        handle_arguments(args);
    }

}

fn handle_arguments(args: Vec<String>) {
    let command = &args[1];
    let project_name = &args[2];
    if command == "new" {
        if make_project(&project_name).is_err(){
            panic!("Making directories failed:");    
        }
    }else{
        unknown_command();
    }
}

fn make_project(project_name: &String) -> std::io::Result<()>{
    make_dirs(project_name)?;
    
    Ok(())
}

fn make_dirs(project_name: &String) -> std::io::Result<()> {
    let mut path_to_write: String = format!("./{}/doc_out/", project_name);
    if !Path::new(&path_to_write).exists() {
        fs::create_dir_all(path_to_write)?;
    }
    path_to_write = format!("./{}/doc_logs/", project_name);
    if !Path::new(&path_to_write).exists() {
        fs::create_dir_all(path_to_write)?;
    }
    path_to_write = format!("./{}/doc_src/", project_name);
    if !Path::new(&path_to_write).exists() {
        fs::create_dir_all(path_to_write)?;
    }
    path_to_write = format!("./{}/images/", project_name);
    if !Path::new(&path_to_write).exists() {
        fs::create_dir_all(path_to_write)?;
    }
    path_to_write = format!("./{}/doc_src/{}.tex", project_name, project_name);
    let mut output = File::create(path_to_write)?;
    let line = "\\documentclass{article}\n\\usepackage{graphicx}\n\\usepackage{indentfirst}\n\\graphicspath{ {./images/} }\n\\title{% \n\tTitle\\\\[2ex] \\large\n\tSubtitle\n}\n\n\\author{Author}\n\n\\begin{document}\n\\maketitle\n\\tableofcontents\n\\newpage\n\\section{Section}\n\\subsection{Subsection}\n\nLorem...\n\\iffalse\n\\begin{center}\n\t\\includegraphics[width=\\textwidth,angle=0]{image_name}\n\\end{center}\n\\fi\n\n\\end{document}";
    write!(output, "{}", line)?;
    
    let mut output = File::create(format!("./{}/Makefile", project_name))?;
    let line = format!("documents: ./doc_src/{}.tex\n\tpdflatex -output-directory=./doc_out/ ./doc_src/{}.tex\n\tpdflatex -output-directory=./doc_out/ ./doc_src/{}.tex\n\tmv -f ./doc_out/{}.aux ./doc_logs/\n\tmv -f ./doc_out/{}.log ./doc_logs/\n\tmv -f ./doc_out/{}.toc ./doc_logs/", project_name, project_name, project_name, project_name, project_name, project_name);
    write!(output, "{}", line)?;
    
    
    Ok(())
}


fn unknown_command(){
    println!("Unknown command");
}