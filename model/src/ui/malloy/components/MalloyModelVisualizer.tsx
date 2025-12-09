import React from 'react';
import { DataModel } from '../../../data/sdsl';

interface MalloyModelVisualizerProps {
  model: DataModel;
}

export const MalloyModelVisualizer: React.FC<MalloyModelVisualizerProps> = ({ model }) => {
  const { dimensions = {}, measures = {} } = model.config;

  return (
    <div className="malloy-model p-4 border rounded-lg bg-gray-50">
      <h3 className="text-lg font-bold mb-4">{model.config.name}</h3>
      <div className="grid grid-cols-2 gap-4">
        <div className="dimensions">
          <h4 className="font-semibold text-gray-700 mb-2">Dimensions</h4>
          <ul className="space-y-1">
            {Object.entries(dimensions).map(([name, def]) => {
               const type = typeof def === 'string' ? 'string' : (def.truncation || 'raw');
               return (
                <li key={name} className="flex items-center text-sm">
                  <span className="w-4 h-4 mr-2 bg-blue-400 rounded-full inline-block"></span>
                  <span>{name}</span>
                  <span className="text-gray-400 ml-2 text-xs">({type})</span>
                </li>
              );
            })}
          </ul>
        </div>
        <div className="measures">
          <h4 className="font-semibold text-gray-700 mb-2">Measures</h4>
          <ul className="space-y-1">
            {Object.entries(measures).map(([name, def]) => (
              <li key={name} className="flex items-center text-sm">
                <span className="w-4 h-4 mr-2 bg-green-400 rounded-full inline-block"></span>
                <span>{name}</span>
                <span className="text-gray-400 ml-2 text-xs">({def.type})</span>
              </li>
            ))}
          </ul>
        </div>
      </div>
    </div>
  );
};
