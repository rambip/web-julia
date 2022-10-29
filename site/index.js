// web-assembly importations
import init from './julia.js';
import {MandelbrotDrawer, JuliaDrawer} from './julia.js';

// init canvas size
let freeW = document.documentElement.clientWidth-40;
let freeH = document.documentElement.clientHeight-70;
const size = Math.round(Math.max(Math.min(freeW, freeH/2), Math.min(freeW/2, freeH)))-15;

// init canvas and draw utilities
const j = document.getElementById("julia"); j.width = size; j.height=size;
const julia_ctx= j.getContext('2d');
julia_ctx.fillStyle = "white";
julia_ctx.textAlign = "center";
julia_ctx.font = "15px arial";
julia_ctx.fillText("Click on the first image, ", size/2, 150);
julia_ctx.fillText("the Mandelbrot set", size/2, 180);
julia_ctx.fillText("at different points", size/2, 210);


const m = document.getElementById("mandel"); m.width = size; m.height = size;
const xminM = -1.7;
const yminM = -1.2;
const scaleM = 2.4;

function setup() {
        let mandelbrot = new MandelbrotDrawer(size, 50, xminM, yminM, scaleM, m.getContext("2d"));
        let julia = new JuliaDrawer(size, 50, -1.5, -1.5, 3.0, julia_ctx);

        mandelbrot.display();
        m.addEventListener("mousedown", draw_julia)

        function draw_julia(event){
                let m_rect = m.getBoundingClientRect();

                // get complex number corresponding to the position of the mouse
                let a = (event.clientX - m_rect.left)*scaleM/size+xminM;
                let b = (event.clientY - m_rect.top)*scaleM/size+yminM;

                document.getElementById("complex").innerText = "C = " + a.toPrecision(3).toString() + "+" + b.toPrecision(3).toString() + "i ";

                julia.set_complex(a, b);
                julia.display();
        }
}


init().then(setup);
