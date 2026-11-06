# rasciify

ASCII-fy images and preview them in terminal

A quick, just-for-fun, rewrite of github.com/EVODelavega/asciify in Rust. Why?

Simply because the original ASCIIfy was written in a single sitting, for fun, and it shows. Because CLI output has a width to height ratio of 1 to 2, the output was skewed something awful. Instead of fixing the issue by just adding a space, I added the width/height flags and left it to the user to specify the desired ratio. This time around, I chose to add the space to maintain the correct aspect ratio.

I also wanted something small and fun to write, myself, by hand, without AI slop, just because I find writing code enjoyable, and I want to keep the skill fresh. Rust is a language I hadn't used in a couple of months, but I really enjoy it, so as a fun little project, writing an improved version of that asciify thing made sense to me.

## Usage

Well, it's a clap CLI tool, so the usage should be quite straightforward:

```bash
$ rasciify -h

Usage: rasciify [OPTIONS]

Options:
      --image <IMAGE>    [default: examples/vim.png]
  -s, --scale <SCALE>    [default: 1]
  -w, --width <WIDTH>    [default: 0]
      --height <HEIGHT>  [default: 0]
  -o, --output <OUTPUT>  [default: stdout]
  -n, --negative         
  -c, --colour           
  -p, --preview          
  -f, --filter <FILTER>  [default: nearest] [possible values: nearest, triangle, catmull-rom, gaussian, lanczos3]
  -h, --help             Print help (see more with '--help')
  -V, --version          Print version
```

* The scale flag is a `f32`, capped to `1.0` (full sized). Larger images can be scaled down (e.g. `rasciify -s 0.1` for a 10% scaled output).
* The width flag specifies the width of the output. The height is calculated to preserve the ratio (e.g. a 1000x500 image with `-w 100` gets resized to 100x50).
* The height flag works much like the width flag, of course.

The cardinality of the scaling parameters are as follows:

* `-s, --scale`: if set to anything other than the default, width and height are ignored
* `-w, --width`: If scale is 1.0, width overrules height
* `--height`: Only factors in if scale == 1.0, and width == 0.

Under the bonnet, width and height are used to calculate the `--scale` value.

### Output format

The `-n, --negative` flag is best illustrated using an example (based on the examples/vim.png image):

Using `$ rasciify -n -w 40`:

```

                                                                                
                                      5 !                                       
                                    5 5 ! !                                     
      Ñ 9 9 9 9 9 9 9 9 9 9 9 9 9   5 : : ! !     0 9 9 9 9 9 9 9 9 9 9 9 9 0   
      Ñ 9 9 9 9 9 9 9 9 9 9 9 9 9   : : : : ! !   0 9 9 9 9 9 9 9 9 9 9 9 9 0   
      Ñ 9 9 9 9 9 9 9 9 9 9 9 9 0   : : : : : !   Ñ 9 9 9 9 9 9 9 9 9 9 9 9 0   
            0 9 9 9 9 9 9 9 0       : : : : : : !     Ñ 9 9 9 9 9 9 9 9 9 0     
            0 9 9 9 9 9 9 9 0   : : : : : : : : :   Ñ Ñ 9 9 9 9 9 9 9 9 0       
            0 9 9 9 9 9 9 9 0   : : : : : : : :   Ñ Ñ 9 9 9 9 9 9 9 9 0         
            0 9 9 9 9 9 9 9 0   : : : : : : :   Ñ Ñ 9 9 9 9 9 9 9 9 0           
            0 9 9 9 9 9 9 9 0   : : : : : :   Ñ Ñ 9 9 9 9 9 9 9 9 0             
            0 9 9 9 9 9 9 9 0   : : : : :   Ñ Ñ 9 9 9 9 9 9 9 9 0               
            0 9 9 9 9 9 9 9 0   : : : :   Ñ Ñ 9 9 9 9 9 9 9 9 0                 
            0 9 9 9 9 9 9 9 0   : : :   Ñ Ñ 9 9 9 9 9 9 9 9 0   !               
            0 9 9 9 9 9 9 9 0   : :   Ñ Ñ 9 9 9 9 9 9 9 9 0   : ! !             
            0 9 9 9 9 9 9 9 0   :   Ñ Ñ 9 9 9 9 9 9 9 9 0   : : : ! !           
        5   0 9 9 9 9 9 9 9 0     Ñ Ñ 9 9 9 9 9 9 9 9 0   : : : : : ! !         
      5 5   0 9 9 9 9 9 9 9 0   Ñ Ñ 9 9 9 9 9 9 9 9 0   : : : : : : : ! !       
    5 5 :   0 9 9 9 9 9 9 9 0 Ñ Ñ 9 9 9 9 9 9 9 9 0   : : : : : : : : : ! !     
  5 5 : :   0 9 9 9 9 9 9 9 0 Ñ 9 9 9 9 9 9 9 9 0   : : : : : : : : : : : ! !   
  = = : :   0 9 9 9 9 9 9 9 Ñ 9 9 9 9 9 9 9 9 0   : : : : : : : : : : : : , ,   
    = = :   0 9 9 9 9 9 9 9 9 9 9 9 9 9 9 9 0   : : : : : : : : : : : : , ,     
      = =   0 9 9 9 9 9 9 9 9 9 9 9 9 9       : : : : : : : : : : : : , ,       
        =   0 9 9 9 9 9 9 9 9 9 9 9 9 9 8 9 9 : : : : : : : : : : : , ,         
            0 9 9 9 9 9 9 9 9 9 9 9 9   9 9   : : : : : : : : : : , ,           
            0 9 9 9 9 9 9 9 9 9 9 9 0   : : : : : : : : : : : : , ,             
            0 9 9 9 9 9 9 9 9 9 9 0         : : : : : : : : : , ,               
            0 9 9 9 9 9 9 9 9 9 0   9 9 9   :   9 9 9     9 9 9     9 9 9       
            0 9 9 9 9 9 9 9 9 0   :   9 9   : : 9 9 9 9 9 9 9 9 9 9 9 9 9       
            0 9 9 9 9 9 9 9 0   : : 9 9 9 : :   9 9       9 9         9 9       
            0 9 9 9 9 9 9 0   : :   9 9   : :   9 9 : ,   9 9       9 9         
            0 9 9 9 9 9 0   : : :   9 9   : : 9 9   , ,   9 9       9 9         
            0 9 9 9 9 0   = = : : 9 9 9 : :   9 9   ,   9 9         9 9         
            0 9 9 9 0       = =   9 9   : :   9 9 ,     9 9       9 9           
            Ñ 9 9 0           =   9 9     : 9 9         9 9       9 9           
                                        : : , ,                                 
                                  = = : : , ,                                   
                                    = = , ,                                     
                                      = ,                                       
                                                                                
```

Compared to `$ rasciify -w 40`:

```

Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ 
Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ c 2 Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ 
Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ c c 2 2 Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ 
Ñ Ñ Ñ   = = = = = = = = = = = = = Ñ c 7 7 2 2 Ñ Ñ 0 = = = = = = = = = = = = 0 Ñ 
Ñ Ñ Ñ   = = = = = = = = = = = = = Ñ 7 7 7 7 2 2 Ñ 0 = = = = = = = = = = = = 0 Ñ 
Ñ Ñ Ñ   = = = = = = = = = = = = 0 Ñ 7 7 7 7 7 2 Ñ   = = = = = = = = = = = = 0 Ñ 
Ñ Ñ Ñ Ñ Ñ Ñ 0 = = = = = = = 0 Ñ Ñ Ñ 7 7 7 7 7 7 2 Ñ Ñ   = = = = = = = = = 0 Ñ Ñ 
Ñ Ñ Ñ Ñ Ñ Ñ 0 = = = = = = = 0 Ñ 7 7 7 7 7 7 7 7 7 Ñ     = = = = = = = = 0 Ñ Ñ Ñ 
Ñ Ñ Ñ Ñ Ñ Ñ 0 = = = = = = = 0 Ñ 7 7 7 7 7 7 7 7 Ñ     = = = = = = = = 0 Ñ Ñ Ñ Ñ 
Ñ Ñ Ñ Ñ Ñ Ñ 0 = = = = = = = 0 Ñ 7 7 7 7 7 7 7 Ñ     = = = = = = = = 0 Ñ Ñ Ñ Ñ Ñ 
Ñ Ñ Ñ Ñ Ñ Ñ 0 = = = = = = = 0 Ñ 7 7 7 7 7 7 Ñ     = = = = = = = = 0 Ñ Ñ Ñ Ñ Ñ Ñ 
Ñ Ñ Ñ Ñ Ñ Ñ 0 = = = = = = = 0 Ñ 7 7 7 7 7 Ñ     = = = = = = = = 0 Ñ Ñ Ñ Ñ Ñ Ñ Ñ 
Ñ Ñ Ñ Ñ Ñ Ñ 0 = = = = = = = 0 Ñ 7 7 7 7 Ñ     = = = = = = = = 0 Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ 
Ñ Ñ Ñ Ñ Ñ Ñ 0 = = = = = = = 0 Ñ 7 7 7 Ñ     = = = = = = = = 0 Ñ 2 Ñ Ñ Ñ Ñ Ñ Ñ Ñ 
Ñ Ñ Ñ Ñ Ñ Ñ 0 = = = = = = = 0 Ñ 7 7 Ñ     = = = = = = = = 0 Ñ 7 2 2 Ñ Ñ Ñ Ñ Ñ Ñ 
Ñ Ñ Ñ Ñ Ñ Ñ 0 = = = = = = = 0 Ñ 7 Ñ     = = = = = = = = 0 Ñ 7 7 7 2 2 Ñ Ñ Ñ Ñ Ñ 
Ñ Ñ Ñ Ñ c Ñ 0 = = = = = = = 0 Ñ Ñ     = = = = = = = = 0 Ñ 7 7 7 7 7 2 2 Ñ Ñ Ñ Ñ 
Ñ Ñ Ñ c c Ñ 0 = = = = = = = 0 Ñ     = = = = = = = = 0 Ñ 7 7 7 7 7 7 7 2 2 Ñ Ñ Ñ 
Ñ Ñ c c 7 Ñ 0 = = = = = = = 0     = = = = = = = = 0 Ñ 7 7 7 7 7 7 7 7 7 2 2 Ñ Ñ 
Ñ c c 7 7 Ñ 0 = = = = = = = 0   = = = = = = = = 0 Ñ 7 7 7 7 7 7 7 7 7 7 7 2 2 Ñ 
Ñ 9 9 7 7 Ñ 0 = = = = = = =   = = = = = = = = 0 Ñ 7 7 7 7 7 7 7 7 7 7 7 7 W W Ñ 
Ñ Ñ 9 9 7 Ñ 0 = = = = = = = = = = = = = = = 0 Ñ 7 7 7 7 7 7 7 7 7 7 7 7 W W Ñ Ñ 
Ñ Ñ Ñ 9 9 Ñ 0 = = = = = = = = = = = = = Ñ Ñ Ñ 7 7 7 7 7 7 7 7 7 7 7 7 W W Ñ Ñ Ñ 
Ñ Ñ Ñ Ñ 9 Ñ 0 = = = = = = = = = = = = = + = = 7 7 7 7 7 7 7 7 7 7 7 W W Ñ Ñ Ñ Ñ 
Ñ Ñ Ñ Ñ Ñ Ñ 0 = = = = = = = = = = = = Ñ = = Ñ 7 7 7 7 7 7 7 7 7 7 W W Ñ Ñ Ñ Ñ Ñ 
Ñ Ñ Ñ Ñ Ñ Ñ 0 = = = = = = = = = = = 0 Ñ 7 7 7 7 7 7 7 7 7 7 7 7 W W Ñ Ñ Ñ Ñ Ñ Ñ 
Ñ Ñ Ñ Ñ Ñ Ñ 0 = = = = = = = = = = 0 Ñ Ñ Ñ Ñ 7 7 7 7 7 7 7 7 7 W W Ñ Ñ Ñ Ñ Ñ Ñ Ñ 
Ñ Ñ Ñ Ñ Ñ Ñ 0 = = = = = = = = = 0 Ñ = = = Ñ 7 Ñ = = = Ñ Ñ = = = Ñ Ñ = = = Ñ Ñ Ñ 
Ñ Ñ Ñ Ñ Ñ Ñ 0 = = = = = = = = 0 Ñ 7 Ñ = = Ñ 7 7 = = = = = = = = = = = = = Ñ Ñ Ñ 
Ñ Ñ Ñ Ñ Ñ Ñ 0 = = = = = = = 0 Ñ 7 7 = = = 7 7 Ñ = = Ñ Ñ Ñ = = Ñ Ñ Ñ Ñ = = Ñ Ñ Ñ 
Ñ Ñ Ñ Ñ Ñ Ñ 0 = = = = = = 0 Ñ 7 7 Ñ = = Ñ 7 7 Ñ = = 7 W Ñ = = Ñ Ñ Ñ = = Ñ Ñ Ñ Ñ 
Ñ Ñ Ñ Ñ Ñ Ñ 0 = = = = = 0 Ñ 7 7 7 Ñ = = Ñ 7 7 = = Ñ W W Ñ = = Ñ Ñ Ñ = = Ñ Ñ Ñ Ñ 
Ñ Ñ Ñ Ñ Ñ Ñ 0 = = = = 0 Ñ 9 9 7 7 = = = 7 7 Ñ = = Ñ W Ñ = = Ñ Ñ Ñ Ñ = = Ñ Ñ Ñ Ñ 
Ñ Ñ Ñ Ñ Ñ Ñ 0 = = = 0 Ñ Ñ Ñ 9 9 Ñ = = Ñ 7 7 Ñ = = W Ñ Ñ = = Ñ Ñ Ñ = = Ñ Ñ Ñ Ñ Ñ 
Ñ Ñ Ñ Ñ Ñ Ñ   = = 0 Ñ Ñ Ñ Ñ Ñ 9 Ñ = = Ñ Ñ 7 = = Ñ Ñ Ñ Ñ = = Ñ Ñ Ñ = = Ñ Ñ Ñ Ñ Ñ 
Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ 7 7 W W Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ 
Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ 9 9 7 7 W W Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ 
Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ 9 9 W W Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ 
Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ 9 W Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ 
Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ Ñ 
```

### Colour

As per the asciify tool, the output can be rendered in colour by adding the `-c, --colour` flag. Internally, I'm just adding `\033[38;2;<R>;<G>;<B>m<ASCII_CHAR> \033m[0m` to the output. It's supported with ASCII output, but that's not the main reason why this flag exists. The main purpose is for the preview mode.

### Preview

Currently the `-p,--preview` flag only works in conjunction with the `-c,--colour` flag. It essentially removes the ASCII character from the output and replaces it with a space. It changes the colour wrapping with `\033[48;...` to set the background colour accordingly. The result is pretty convincing all the same.

Using the default `-f,--filter` flag (nearest), is the fastest, but quite blocky. It's perfectly adequate for ASCII output. If the result is a sub-optimal, feel free to use the slower `-f triangle` and so on. The order in which they are listed in the help output is from fastest to slowest. I find `-f triangle` is a decent improvement, and for some really busy images catmull-rom offers slight improvements, but beyond that, the scaling is just ridiculously slow IMO. If you find it worth having, feel free to use it, though.

The `-n,--negative` flag was fixed to actually generate a negative image while in preview mode, but only in preview mode, in ASCII mode it still maps the colours the same way, but flips the character mappings. Note that the output of this `-n,--negative` flag in preview mode isn't a _"true"_ negative, but rather a standard inversion. Light intensity is not inverted, but overall the output is recogniseable as a negative.
