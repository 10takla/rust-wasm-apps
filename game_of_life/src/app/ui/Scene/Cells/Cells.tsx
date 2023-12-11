import {Box} from "@react-three/drei";
import {ReactElement} from "react";
import {Cell} from "../../../../../../pkg";

interface CellsProps {
    size: number,
    pos: number[]
    width: number,
    height: number,
    x: number,
    y: number,
    z: number,
    cells: number[]
}

export const Cells = (props: CellsProps) => {
    const {
        size, pos, x, y, z, width, height,cells
    } = props;
    const getIndex = (x: number, y: number, z: number) => {
        return (x * width + y) * height + z;
    }
    return (
        <Box
            args={[size, size, size]}
            position={pos}
        >
            {
                cells[getIndex(x, y, z)] === Cell.Alive ? (
                    <meshStandardMaterial color={'white'} wireframe={false}/>
                ) : (
                    <meshStandardMaterial transparent={true} opacity={0.2} color={'red'} wireframe={false}/>
                )
            }
        </Box>
    )
}

