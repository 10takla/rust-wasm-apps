import {Canvas} from "@react-three/fiber";
import {Box, OrbitControls, Stats} from "@react-three/drei";
import cls from './Scene.module.scss';
import {Cell, Pos, Universe} from "my-planet";
import {memory} from "my-planet/my_planet_bg.wasm";
import {useCallback, useEffect, useMemo, useState} from "react";

interface SceneProps {
    trigger: number
    isStart: boolean
    speed: number
    resolution: [number, number, number]
    size?: number
}

export const Scene = (props: SceneProps) => {
    const {
        trigger,
        isStart,
        speed,
        resolution,
        size = 0.4
    } = props;
    const S = size + 0.05;
    const [universe, width, height, depth] = useMemo(() => {
        let universe = Universe.new(Pos.new(...resolution))
        universe.set_cell(Pos.new(1, 2, 2))
        universe.set_cell(Pos.new(2, 2, 2))
        universe.set_cell(Pos.new(3, 2, 2))
        return [
            universe,
            universe.width(),
            universe.height(),
            universe.depth()
        ]
    }, [resolution])
    const f = () => new Uint8Array(memory.buffer, universe.cells(), width * height * depth)
    const [cells, setCells] = useState<Uint8Array>(f());
    const [inter, setInter] = useState<NodeJS.Timeout>();
    const setInterFunc = useCallback(() => {
        setInter(
            setInterval(() => {
                universe.tick()
                setCells(f())
            }, speed)
        )
    }, [universe, speed]);
    useEffect(() => {
        if (inter) {
            clearInterval(inter)
            setInterFunc()
        }
    }, [speed]);
    useEffect(() => {
        clearInterval(inter)
        if (isStart) {
            setInterFunc()
        }
        return () => clearInterval(inter)
    }, [isStart]);



    useEffect(() => {
        if (trigger) {
            universe.tick()
            setCells(f())
        }
    }, [trigger]);

    const getIndex = (x: number, y: number, z: number) => {
        return (x * width + y) * height + z;
    }
    return (
        <Canvas
            className={cls.Scene}
        >
            <Stats/>
            <ambientLight intensity={1}/>
            <pointLight position={[width * S + 0.5, height * S + 0.5, depth * S + 0.5]} intensity={100}/>
            <OrbitControls enablePan={false}/>
            {Array(width).fill(null).map((_, x) => (
                Array(height).fill(null).map((__, y) => (
                    Array(depth).fill(null).map((___, z) => (
                        cells[getIndex(x, y, z)] === Cell.Alive &&
                            <Box
                                key={`${x} ${y} ${z}`}
                                args={[size, size, size]}
                                position={[(z - depth / 2) * S, (x - width / 2) * S, (y - height / 2) * S]}
                            >
                                <meshStandardMaterial color={'white'} wireframe={false}/>
                            </Box>
                        )
                    ))
                )))}
        </Canvas>
    )
}
