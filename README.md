# bigcsv2md

Custom utility for converting csv files into markdown tables

## Simple Example

fruit.csv:

```text
,Apple,Orange,Strawberry,Lemon,Pear,Blueberry,Grape
Weight,120,150,20,100,130,5,12
Color,Green,Orange,Red,Yellow,Green,Dark Blue,Light Green
```

command:

```text
bigcsv2md fruit.csv -o fruit.md
```

fruit.md

```text
||Apple|Orange|Strawberry|Lemon|Pear|Blueberry|Grape|
|-|-----|------|----------|-----|----|---------|-----|
|Weight|120|150|20|100|130|5|12|
|Color|Green|Orange|Red|Yellow|Green|Dark Blue|Light Green|
```

## Column Splitting Example

fruit.csv:

```text
,Apple,Orange,Strawberry,Lemon,Pear,Blueberry,Grape
Weight,120,150,20,100,130,5,12
Color,Green,Orange,Red,Yellow,Green,Dark Blue,Light Green
```

command:

```text
bigcsv2md fruit.csv -o fruit.md --csplit 3
```

fruit.md

```text
||Apple|Orange|
|-|-----|------|
|Weight|120|150|
|Color|Green|Orange|

|Strawberry|Lemon|Pear|
|----------|-----|----|
|20|100|130|
|Red|Yellow|Green|

|Blueberry|Grape|
|---------|-----|
|5|12|
|Dark Blue|Light Green|
```

## Column Splitting with Row Headers Example

fruit.csv:

```text
,Apple,Orange,Strawberry,Lemon,Pear,Blueberry,Grape
Weight,120,150,20,100,130,5,12
Color,Green,Orange,Red,Yellow,Green,Dark Blue,Light Green
```

command:

```text
bigcsv2md fruit.csv -o fruit.md --csplit 3 --rheaders
```

fruit.md

```text
| |Apple|Orange|Strawberry|
|-|-----|------|----------|
|Weight|120|150|20|
|Color|Green|Orange|Red|

| |Lemon|Pear|Blueberry|
|-|-----|----|---------|
|Weight|100|130|5|
|Color|Yellow|Green|Dark Blue|

| |Grape|
|-|-----|
|Weight|12|
|Color|Light Green|
```
