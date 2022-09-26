version 1.0

workflow munge_for_metastaar {
    input {
        File in_file
        String out_file_name
    }
    call munge {
        input:
            in_file = in_file,
            out_file_name = out_file_name
    }
}

task munge {
    input {
        File in_file
        String out_file_name
    }
    runtime {
        docker: "gcr.io/nitrogenase-docker/tsv:0.0.2"
    }
    command <<<
        echo "Starting ..."
        echo "munge_for_metastaar()" > script.n
        echo "=== Start of script.n ==="
        cat script.n
        echo "=== End of script.n ==="
        tsv nitro script.n -i ~{in_file} -o ~{out_file_name}
        echo "Done!"
    >>>
    output {
        File out_file = out_file_name
    }
}