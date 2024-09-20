# WTF
Just me playing around with simple rust

# Main purpose
Bump version number in Chart.yaml for a helm chart

# TO install
cargo install --path .

helm-chart-version-bumper

alias bump="helm-chart-version-bumper"

NB! There must be a Chart.yaml in your current directory when running this

# To run
Stand in directory which has Chart.yaml
```
bump 

//or

helm-chart-version-bumper
```

## To check
```
git diff
```