#!/bin/bash

pushd "$(dirname $0)"

CLASSPATH=$(realpath .)
JAR_CACHE=$(realpath "../../jar_cache")

for jarfile in ${JAR_CACHE}/*.jar ; do
    CLASSPATH="${CLASSPATH}:$(realpath ${jarfile})"
done

export CLASSPATH
echo ${CLASSPATH}

javac ./Item.java
javac ./JasperTableExample_Items.java
java JasperTableExample_Items

javac ./JasperTableExample_Json.java
java JasperTableExample_Json

popd
