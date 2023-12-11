declare module '*.scss' {
    interface IClassNames {
        [className: string]: string
    }
    const classNames: IClassNames;
    export = classNames;
}

declare module '*.svg' {
    import React from "react";
    const el: React.VFC<React.SVGProps<SVGSVGElement>>
    export = el
}
