import QtQuick 2.15
import QtQuick.Controls 2.15
import FluentUI 1.0

FluStatusLayout {
    property url source: ""
    property bool lazy: false
    color:"transparent"
    id:control
    onErrorClicked: {
        reload()
    }
    Component.onCompleted: {
        if(!lazy){
            loader.source = control.source
        }
    }
    FluLoader{
        id:loader
        anchors.fill: parent
        asynchronous: true
        onStatusChanged: {
            if(status === Loader.Error){
                control.statusMode = FluStatusLayoutType.Error
            }else if(status === Loader.Loading){
                control.statusMode = FluStatusLayoutType.Loading
            }else{
                control.statusMode = FluStatusLayoutType.Success
            }
        }
    }
    function reload(){
        var timestamp = Date.now();
        loader.source = control.source+"?"+timestamp
    }
    function itemLodaer(){
        return loader
    }
}
