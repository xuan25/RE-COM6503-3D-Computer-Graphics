:: Require ImageMagick installed

@echo off
for %%i in (*.jpg) do (
    echo %%~i
    convert -flip %%~i %%~ni_flipped%%~xi
)