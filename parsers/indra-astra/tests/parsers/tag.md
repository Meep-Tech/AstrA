# Units
## Named
```
#test
```
- tag [0, 4]
  - name [1, 4]

## Named & Input
```
>#test
```
- input-tag [0, 5] #tag
  - name [1, 4]

## Named & Output
```
>>#test
```
- output-tag [0, 6] #tag
  - name [2, 4]

# Patterns
## Named
```
#{name}
```
- tag
  - name

## Named & Input
```
>#{name}
```
- input-tag #tag
  - name

## Named & Output
```
>>#{name}
```
- output-tag #tag
  - name
