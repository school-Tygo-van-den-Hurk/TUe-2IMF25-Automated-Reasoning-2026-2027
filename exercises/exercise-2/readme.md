# PCB layout

## How to Run

TODO...

## Assignment

Modern foldable phones contain multiple printed circuit boards (PCBs), usually
one on each side of the hinge. A phone manufacturer needs to design the
internal layout for their next flagship model. And so, they have to fit some
chips on these PCBs. What makes it challenging is that the PCBs are odd-shaped
in order to fit tightly around the battery and camera. The design should meet
the following constraints:

- There are 14 chips that need to fit on the PCBs, each chip is placed exactly
  once. The dimensions of the PCBs and the size of each chip is given.
- Three of the chips can become warm under load. To prevent overheating, their
  centres should not be closer than 20 length units in either the x or y
  direction.
- Chips may be rotated 90 degrees, but may not overlap.

<table>
  <thead>
    <tr>
      <th>Chips</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>10</th>
      <th>11</th>
      <th>12</th>
      <th>13</th>
      <th>14</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <th>width</th>
      <td>5</td><td>5</td><td>4</td><td>4</td><td>5</td><td>3</td><td>7</td>
      <td>6</td><td>6</td><td>4</td><td>6</td><td>5</td><td>6</td><td>5</td>
    </tr>
    <tr>
      <th>height</th>
      <td>6</td><td>6</td><td>6</td><td>10</td><td>7</td><td>7</td><td>7</td>
      <td>10</td><td>12</td><td>10</td><td>9</td><td>11</td><td>10</td>
      <td>10</td>
    </tr>
    <tr>
      <th>hot</th>
      <td>X</td><td>X</td><td></td><td></td><td></td><td>X</td><td></td>
      <td></td><td></td><td></td><td></td><td></td><td></td><td></td>
    </tr>
  </tbody>
</table>

### Sub-question a

Give a layout of the PCBs following the above constraints.

### Sub-question b

To boost the spec sheet, the camera department would like to integrate more
cameras. This will increase the width of the camera cutout by 4 length units
for each additional camera. What happens if one camera is added?

### Sub-question c

And what if three cameras are added?
