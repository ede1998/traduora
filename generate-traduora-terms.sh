#!/bin/bash

# $1 = number of terms
# $2... = output locale file names

for file in "${@:2}"; do
	if [ -e "$file" ]; then
		echo "File $file exists."
		exit;
	else
		echo "{" > "$file";
	fi
done

for N in `seq "$1"`; do
	term=`shuf -n "5" wordlist.txt | tr '\n' '.' | sed 's/.$//'`;
	for file in "${@:2}"; do
		translation=`shuf -n "3" wordlist.txt | tr '\n' ' ' | sed 's/.$//'`;
		if [[ N -eq "$1" ]]; then
			echo '    "'"$term"'": "'"$translation"'"' >> "$file";
		else
			echo '    "'"$term"'": "'"$translation"'",' >> "$file";
		fi
	done
done

for file in "${@:2}"; do
	echo "}" >> "$file";
done
