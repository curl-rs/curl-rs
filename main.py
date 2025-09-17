import subprocess

# Run `curl --help all` and pipe through sed to extract lines containing --arguments
cmd = "curl --help all | sed -n 's/^\\s*\\(--[^[:space:],]*\\).*/\\1/p'"
result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
output = result.stdout

# Extract all arguments (unique)
args = set(output.splitlines())

# Sort arguments alphabetically and format as markdown checkboxes
args = sorted(args)
formatted_args = [f"- [ ] {arg[2:]}" for arg in args]  # remove leading '--'

# Write to markdown file
with open("curl_arguments.md", "w") as f:
    f.write("# Curl Arguments\n\n")
    f.write("\n".join(formatted_args))

print("curl_arguments.md file created with all curl arguments.")
