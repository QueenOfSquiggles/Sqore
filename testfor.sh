mkdir temp
for i in $(ls target/**/*sqore*.* );
do
	echo $i
	cp $i temp/
done;