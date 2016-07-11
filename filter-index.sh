#!/bin/sh
perl -i -ne 'print unless /^searchIndex\["(?!byteorder|positioned_io).*"\]/' "$@"
