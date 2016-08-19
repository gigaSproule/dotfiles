#! /bin/bash
for remote in `git branch -r | grep -v master `; do git checkout --track $remote ; done
git remote rm origin
git remote add origin $1
git remote show origin
git push origin '*:*'

