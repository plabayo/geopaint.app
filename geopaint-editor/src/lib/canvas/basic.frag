// Plabayo — geopaint.app
// Copyright (C) 2022 Glen Henri J. De Cauwsemaecker
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

precision mediump float;

uniform float u_time;

void main() {
    float r = sin(u_time * 0.0003);
    float g = sin(u_time * 0.0005);
    float b = sin(u_time * 0.0007);

    gl_FragColor = vec4(r, g, b, 1.0);
}