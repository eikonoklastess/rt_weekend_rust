output="output.txt"
> "$output" # clear file

for file in src/*; do
  [ -f "$file" ] || continue
  echo "=== $(basename "$file") ===" >> "$output"
  cat "$file" >> "$output"
  echo -e "\n" >> "$output"
done
