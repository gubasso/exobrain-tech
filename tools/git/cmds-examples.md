# Git Commands Examples

Graphical log

```sh
git log --oneline --graph --decorate -n 15 master

git log --graph --decorate --oneline --simplify-by-decoration \
  --branches=master --tags \
  --max-count=50

git log --graph --decorate --oneline \
  --branches='release/*' --tags \
  --max-count=50
```
